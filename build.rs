use cxx_qt_build::{CppFile, CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("org.kde.rhesis").qml_file("src/qml/Main.qml"))
        .qt_module("Gui")
        .qt_module("Quick")
        .cpp_file(CppFile::from("src/cpp/helper.h"))
        .files(["src/bridge.rs"])
        .build();
}
