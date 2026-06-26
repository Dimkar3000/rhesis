use cxx_qt_build::{CppFile, CxxQtBuilder, QmlModule};

fn main() {
    unsafe {
        CxxQtBuilder::new_qml_module(
            QmlModule::new("org.dimkar.rhesis")
                .qml_file("src/interop/qml/Root.qml")
                .qml_file("src/interop/qml/MainPage.qml")
                .qml_file("src/interop/qml/SecondPage.qml"),
        )
        .qt_module("Gui")
        .qt_module("Quick")
        .cpp_file(CppFile::from("src/interop/cpp/helper.h"))
        .files(["src/interop/bridge.rs"])
        .cc_builder(|a| {
            a.flag_if_supported("-w"); // Disabled warning from qt code base. We cannot fix those.
        })
        .build();
    }
}
