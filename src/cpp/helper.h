#pragma once
#include <QTextCharFormat>
#include <memory>

inline std::unique_ptr<QTextCharFormat> newQTextCharFormat() noexcept {
    return std::make_unique<QTextCharFormat>();
}
