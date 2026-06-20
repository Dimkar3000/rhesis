#[cxx_qt::bridge]
pub mod ffi {
    #[derive(Default, Debug, Clone, PartialEq)]
    pub struct Recommendation {
        range: Range,
        value: String,
        color: String,
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    pub struct Range {
        start: i32,
        length: i32,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qlist.h");
        type QList_QString = cxx_qt_lib::QList<QString>;
    }

    unsafe extern "C++Qt" {
        include!(<QtGui/QSyntaxHighlighter>);
        #[qobject]
        type QSyntaxHighlighter;
    }

    unsafe extern "C++Qt" {
        include!(<QtQuick/QQuickTextDocument>);
        #[qobject]
        type QQuickTextDocument;
    }

    unsafe extern "C++" {
        fn textDocument(self: &QQuickTextDocument) -> *mut QTextDocument;
    }

    unsafe extern "C++" {
        include!("helper.h");
        include!(<QtGui/QTextCharFormat>);
        type QTextCharFormat;
        fn newUnderlinedFormat(colorName: &QString) -> UniquePtr<QTextCharFormat>;

        unsafe fn replaceTextInDocument(
            doc: *mut QTextDocument,
            start: i64,
            end: i64,
            replacement: &QString,
        );
    }

    unsafe extern "C++" {
        include!(<QtGui/QTextDocument>);
        type QTextDocument;
    }

    extern "RustQt" {
        #[qobject]
        #[base = QSyntaxHighlighter]
        #[qml_element]
        type CustomHighlighter = super::RustQSyntaxHighlighter;

        #[qobject]
        #[qml_element]
        type AsyncHelper = super::AsyncHelperRust;

        #[qinvokable]
        fn start_async_worker(self: Pin<&mut AsyncHelper>);

        #[qinvokable]
        fn text_area_changed(self: Pin<&mut AsyncHelper>, text: QString);

    }

    impl cxx_qt::Threading for AsyncHelper {}
    impl cxx_qt::Threading for CustomHighlighter {}

    extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "highlightBlock"]
        fn highlight_block(self: Pin<&mut CustomHighlighter>, text: &QString);

        #[qinvokable]
        #[cxx_name = "getSuggestions"]
        fn get_suggestions(
            self: Pin<&mut CustomHighlighter>,
            start: i32,
            length: i32,
        ) -> QList_QString;

        #[qinvokable]
        #[cxx_name = "findRecommendation"]
        fn find_recommendation(self: Pin<&mut CustomHighlighter>, pos: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "replaceWord"]
        fn replace_word(
            self: Pin<&mut CustomHighlighter>,
            start: i64,
            end: i64,
            replacement: &QString,
        );

        #[qinvokable]
        #[cxx_name = "startMessageThread"]
        unsafe fn start_message_thread(self: Pin<&mut CustomHighlighter>, helper: *mut AsyncHelper);

        #[qinvokable]
        #[cxx_name = "setTextDocument"]
        unsafe fn set_text_document(
            self: Pin<&mut CustomHighlighter>,
            doc: *mut QQuickTextDocument,
        );
    }

    unsafe extern "RustQt" {
        #[inherit]
        #[cxx_name = "rehighlight"]
        fn rehighlight(self: Pin<&mut CustomHighlighter>);

        #[inherit]
        #[cxx_name = "setFormat"]
        fn set_format(
            self: Pin<&mut CustomHighlighter>,
            start: i32,
            length: i32,
            format: &QTextCharFormat,
        );

        #[inherit]
        #[cxx_name = "setDocument"]
        unsafe fn set_document(self: Pin<&mut CustomHighlighter>, doc: *mut QTextDocument);

        #[inherit]
        #[cxx_name = "document"]
        unsafe fn document(self: Pin<&mut CustomHighlighter>) -> *mut QTextDocument;
    }
}

use cxx::UniquePtr;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::{pin::Pin, time::Duration};
use tokio::{sync::watch, task::JoinHandle, time::sleep};

use crate::interop::bridge::ffi::{newUnderlinedFormat, Recommendation};
use crate::languatool::{
    client::LanguageToolClient,
    service::{Message, Suggestion},
};

pub struct AsyncHelperRust {
    pub message_sender: watch::Sender<Message>,
    pub message_receiver: watch::Receiver<Message>,
    pub suggestion_sender: watch::Sender<Suggestion>,
    pub suggestion_receiver: watch::Receiver<Suggestion>,

    suppress_next: bool,
    handle: Option<JoinHandle<()>>,
}

impl Default for AsyncHelperRust {
    fn default() -> Self {
        let (message_sender, message_receiver) = watch::channel::<Message>(Message("".to_string()));
        let (suggestion_sender, suggestion_receiver) =
            watch::channel::<Suggestion>(Suggestion(vec![]));

        Self {
            message_sender,
            message_receiver,
            suggestion_sender,
            suggestion_receiver,
            suppress_next: false,
            handle: None,
        }
    }
}

impl ffi::AsyncHelper {
    fn start_async_worker(self: Pin<&mut Self>) {
        let mut message_receiver = self.message_receiver.clone();
        let suggestion_sender = self.suggestion_sender.clone();

        self.rust_mut().handle = Some(tokio::spawn(async move {
            let mut last_text = String::new();
            loop {
                let _ = message_receiver.changed().await;

                loop {
                    let debounce = sleep(Duration::from_millis(300));
                    tokio::pin!(debounce);
                    tokio::select! {
                        _ = &mut debounce => break,
                        _ = message_receiver.changed() => {}
                    }
                }

                let message = message_receiver.borrow().clone();
                let Message(text) = message;
                if text == last_text || text.trim().is_empty() {
                    continue;
                }
                last_text = text.clone();

                let suggestions = LanguageToolClient::get_recommendation(text).await;
                let _ = suggestion_sender.send(Suggestion(suggestions));
            }
        }));
    }

    fn text_area_changed(self: Pin<&mut Self>, text: QString) {
        if self.rust().suppress_next {
            self.rust_mut().suppress_next = false;
            return;
        }
        let _ = self.message_sender.send(Message(text.to_string()));
    }
}
impl Drop for ffi::AsyncHelper {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }
    }
}

impl Drop for ffi::CustomHighlighter {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }
    }
}

#[derive(Default)]
pub struct RustQSyntaxHighlighter {
    recommendations: Vec<Recommendation>,

    handle: Option<JoinHandle<()>>,
}

impl ffi::CustomHighlighter {
    pub fn highlight_block(mut self: Pin<&mut Self>, _text: &QString) {
        println!("highlight_block");
        let ranges: Vec<(i32, i32, UniquePtr<ffi::QTextCharFormat>)> = self
            .recommendations
            .iter()
            .map(|r| {
                let color = QString::from(r.color.clone());
                let format = newUnderlinedFormat(&color);
                let start = r.range.start;
                let length = r.range.length;
                (start, length, format)
            })
            .collect();

        for (start, length, format) in ranges {
            self.as_mut().set_format(start, length, &format);
        }
    }

    pub fn get_suggestions(self: Pin<&mut Self>, start: i32, length: i32) -> ffi::QList_QString {
        self.recommendations
            .iter()
            .filter(|r| r.range.start >= start && r.range.length <= length)
            .take(5)
            .map(|x| QString::from(&x.value))
            .collect::<ffi::QList_QString>()
    }

    pub fn replace_word(mut self: Pin<&mut Self>, start: i64, end: i64, replacement: &QString) {
        self.as_mut().rust_mut().recommendations.clear();
        unsafe {
            let doc = self.document();
            if !doc.is_null() {
                ffi::replaceTextInDocument(doc, start, end, replacement);
            }
        }
    }

    pub fn find_recommendation(self: Pin<&mut Self>, pos: i32) -> QString {
        for r in &self.recommendations {
            let start = r.range.start;
            let end = r.range.start + r.range.length;
            if pos >= start && pos < end {
                return QString::from(format!("{};{}", start, end));
            }
        }
        QString::default()
    }

    pub fn set_text_document(self: Pin<&mut Self>, doc: *mut ffi::QQuickTextDocument) {
        let text_doc = unsafe { (*doc).textDocument() };
        unsafe { self.set_document(text_doc) };
    }

    pub fn start_message_thread(self: Pin<&mut Self>, helper: *mut ffi::AsyncHelper) {
        println!("started rec receiver thread");
        let helper = unsafe { &mut *helper };
        let mut receiver = helper.suggestion_receiver.clone();
        let qt_thread = self.qt_thread();

        self.rust_mut().handle = Some(tokio::spawn(async move {
            loop {
                let _ = receiver.changed().await;
                let r = receiver.clone();
                let _ = qt_thread.queue(move |mut a: Pin<&mut ffi::CustomHighlighter>| {
                    let suggestions = r.borrow().0.clone();
                    a.as_mut().rust_mut().recommendations = suggestions;
                    a.as_mut().rehighlight();
                });
            }
        }));
    }
}
