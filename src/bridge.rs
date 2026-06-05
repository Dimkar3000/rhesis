#[cxx_qt::bridge]
mod bridge {
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
        fn newQTextCharFormat() -> UniquePtr<QTextCharFormat>;
        fn setFontUnderline(self: Pin<&mut QTextCharFormat>, underline: bool);
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
        fn get_suggestions(self: Pin<&mut CustomHighlighter>, word: &QString) -> QString;
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
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "setTextDocument"]
        unsafe fn set_text_document(self: Pin<&mut CustomHighlighter>, doc: *mut QQuickTextDocument);
    }
}

use cxx_qt_lib::QString;
use std::pin::Pin;

#[derive(Default)]
pub struct RustQSyntaxHighlighter;

impl bridge::CustomHighlighter {
    pub fn highlight_block(mut self: Pin<&mut Self>, text: &QString) {
        let s = text.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if !bytes[i].is_ascii_alphabetic() {
                i += 1;
                continue;
            }
            let start = i;
            while i < len && bytes[i].is_ascii_alphanumeric() {
                i += 1;
            }
            if s[start..i].starts_with('a') || s[start..i].starts_with('A') {
                let mut fmt = bridge::newQTextCharFormat();
                if let Some(mut fmt) = fmt.as_mut() {
                    fmt.as_mut().setFontUnderline(true);
                    self.as_mut()
                        .set_format(start as i32, (i - start) as i32, &*fmt);
                }
            }
        }
    }

    pub fn get_suggestions(self: Pin<&mut Self>, word: &QString) -> QString {
        let word = word.to_string();
        let suggestions = vec![
            format!("super_{}", word),
            format!("mega_{}", word),
            format!("ultra_{}", word),
        ];
        QString::from(&suggestions.join(";"))
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
