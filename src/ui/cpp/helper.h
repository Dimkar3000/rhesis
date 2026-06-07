#pragma once
#include <QColor>
#include <QTextCharFormat>
#include <QTextCursor>
#include <QTextDocument>
#include <QGuiApplication>
#include <QPalette>
#include <memory>

inline std::unique_ptr<QTextCharFormat> newQTextCharFormat() noexcept {
    return std::make_unique<QTextCharFormat>();
}

inline std::unique_ptr<QTextCharFormat> newUnderlinedFormat(const QString& colorName) noexcept {
    auto fmt = std::make_unique<QTextCharFormat>();
    QColor textColor = QGuiApplication::palette().color(QPalette::Text);
    fmt->setForeground(QBrush(textColor));
    fmt->setFontUnderline(true);
    fmt->setUnderlineColor(QColor(colorName));
    return fmt;
}

inline void replaceTextInDocument(QTextDocument* doc, int start, int end, const QString& replacement) {
    QTextCursor cursor(doc);
    cursor.setPosition(start);
    cursor.setPosition(end, QTextCursor::KeepAnchor);
    cursor.insertText(replacement);
}
