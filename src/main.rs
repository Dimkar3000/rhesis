use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};

use cxx_qt_lib_extras::QApplication;
use lazy_static::lazy_static;

use std::env;

use crate::interop::bridge;

mod interop;
mod languatool;

lazy_static! {
    static ref NAMESPACE: QString = QString::from("org.dimkar.rhesis");
    static ref MAIN_QML_FILE_PATH: QUrl = QUrl::from(&format!(
        "qrc:/qt/qml/{}/src/interop/qml/Main.qml",
        NAMESPACE.to_string().replace(".", "/")
    ));
    static ref LOGO_PATH: QString = QString::from("logo.png");
}

#[tokio::main()]
async fn main() {
    run_ui();
}

fn run_ui() {
    env_logger::init();

    let mut app = QApplication::new();

    let mut engine = QQmlApplicationEngine::new();

    // To associate the executable to the installed desktop file
    QGuiApplication::set_desktop_file_name(&NAMESPACE);
    // To ensure the style is set correctly
    let style = env::var("QT_QUICK_CONTROLS_STYLE");
    if style.is_err() {
        QQuickStyle::set_style(&NAMESPACE);
    }

    if let Some(engine) = engine.as_mut() {
        engine.load(&MAIN_QML_FILE_PATH);
    }

    log::info!("Initialized");
    if let Some(mut app) = app.as_mut() {
        bridge::ffi::appSetWindowIcon(app.as_mut(), &LOGO_PATH);
        app.exec();
    }
}
