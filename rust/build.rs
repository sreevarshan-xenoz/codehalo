use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "CodeHalo",
            rust_files: &["src/qt/bridge.rs"],
            qml_files: &[],
            ..Default::default()
        })
        .build();
}
