#pragma once
#include <QColor>
#include <QTextCharFormat>
#include <QTextCursor>
#include <QTextDocument>
#include <QApplication>
#include <QPalette>
#include <QIcon>
#include <QSettings>
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

inline void replaceTextInDocument(QTextDocument* doc, int64_t start, int64_t end, const QString& replacement) {
    QTextCursor cursor(doc);
    cursor.setPosition(start);
    cursor.setPosition(end, QTextCursor::KeepAnchor);
    cursor.insertText(replacement);
}

inline void appSetWindowIcon(QApplication& app, const QString& path) {
  app.setWindowIcon(QIcon(path));
}

inline void setupIconTheme() {
    QStringList paths = QIcon::themeSearchPaths();
    paths.prepend(":/icons");
    QIcon::setThemeSearchPaths(paths);
}
