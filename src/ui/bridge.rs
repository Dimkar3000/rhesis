#[cxx_qt::bridge]
pub mod bridge {
    #[derive(Default, Debug)]
    pub struct Recomendation {
        start: i32,
        end: i32,
        value: String,
        color: String,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!(<QtGui/QSyntaxHighlighter>);
        #[qobject]
        type QSyntaxHighlighter;
    }

    unsafe extern "C++" {
        include!("helper.h");
        include!(<QtGui/QTextCharFormat>);
        type QTextCharFormat;
        fn newUnderlinedFormat(colorName: &QString) -> UniquePtr<QTextCharFormat>;
        unsafe fn replaceTextInDocument(
            doc: *mut QTextDocument,
            start: i32,
            end: i32,
            replacement: &QString,
        );
    }

    unsafe extern "C++" {
        include!(<QtGui/QTextDocument>);
        type QTextDocument;
    }

    unsafe extern "C++" {
        include!(<QtQuick/QQuickTextDocument>);
        type QQuickTextDocument;
        fn textDocument(self: &QQuickTextDocument) -> *mut QTextDocument;
    }

    extern "RustQt" {
        #[qobject]
        #[base = QSyntaxHighlighter]
        #[qml_element]
        type CustomHighlighter = super::RustQSyntaxHighlighter;
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "highlightBlock"]
        fn highlight_block(self: Pin<&mut CustomHighlighter>, text: &QString);
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "getSuggestions"]
        fn get_suggestions(self: Pin<&mut CustomHighlighter>, start: i32, end: i32) -> QString;
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "replaceWord"]
        fn replace_word(
            self: Pin<&mut CustomHighlighter>,
            start: i32,
            end: i32,
            replacement: &QString,
        );
    }

    unsafe extern "RustQt" {
        #[inherit]
        #[cxx_name = "setFormat"]
        fn set_format(
            self: Pin<&mut CustomHighlighter>,
            start: i32,
            count: i32,
            format: &QTextCharFormat,
        );

        #[inherit]
        #[cxx_name = "setDocument"]
        unsafe fn set_document(self: Pin<&mut CustomHighlighter>, doc: *mut QTextDocument);

        #[inherit]
        #[cxx_name = "document"]
        unsafe fn document(self: Pin<&mut CustomHighlighter>) -> *mut QTextDocument;
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "setTextDocument"]
        unsafe fn set_text_document(
            self: Pin<&mut CustomHighlighter>,
            doc: *mut QQuickTextDocument,
        );
    }
}

use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use std::pin::Pin;

use crate::languatool::client::LanguageToolClient;

#[derive(Default)]
pub struct RustQSyntaxHighlighter {
    recommendation: Vec<bridge::Recomendation>,
}

impl bridge::CustomHighlighter {
    pub fn highlight_block(mut self: Pin<&mut Self>, text: &QString) {
        let recommendations = LanguageToolClient::get_recomendation(text.to_string());

        for rec in &recommendations {
            let mut fmt = bridge::newUnderlinedFormat(&QString::from(&rec.color));
            if let Some(fmt) = fmt.as_mut() {
                self.as_mut()
                    .set_format(rec.start, rec.end - rec.start, &fmt);
            }
        }

        self.as_mut().rust_mut().recommendation = recommendations;
    }

    pub fn get_suggestions(self: Pin<&mut Self>, start: i32, end: i32) -> QString {
        let iter = self
            .as_ref()
            .recommendation
            .iter()
            .filter(|r| r.start >= start && r.end <= end)
            .map(|x| x.value.clone())
            .collect::<Vec<_>>();

        QString::from(&iter.join(";"))
    }

    pub fn replace_word(self: Pin<&mut Self>, start: i32, end: i32, replacement: &QString) {
        unsafe {
            let doc = self.document();
            if !doc.is_null() {
                bridge::replaceTextInDocument(doc, start, end, replacement);
            }
        }
    }

    pub fn set_text_document(self: Pin<&mut Self>, doc: *mut bridge::QQuickTextDocument) {
        unsafe {
            if let Some(d) = doc.as_ref() {
                let text_doc = d.textDocument();
                self.set_document(text_doc);
            }
        }
    }
}
