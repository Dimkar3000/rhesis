use cxx_qt_build::{CppFile, CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("org.kde.rhesis").qml_file("src/ui/qml/Main.qml"))
        .qt_module("Gui")
        .qt_module("Quick")
        .cpp_file(CppFile::from("src/ui/cpp/helper.h"))
        .files(["src/ui/bridge.rs"])
        .build();
}
