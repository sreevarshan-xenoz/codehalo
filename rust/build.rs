use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    let qml_files: &[&str] = &[];
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "CodeHalo",
            rust_files: &["src/qt/bridge.rs"],
            qml_files,
            ..Default::default()
        })
        .build();

    #[cfg(windows)]
    {
        if let Ok(qmake) = std::env::var("QMAKE") {
            let p = std::path::Path::new(&qmake);
            if let Some(bin_dir) = p.parent() {
                if let Some(prefix) = bin_dir.parent() {
                    let lib_dir = prefix.join("lib");
                    println!("cargo:rustc-link-search=native={}", lib_dir.display());
                }
            }
        }
        println!("cargo:rustc-link-search=native=E:/Qt/6.7.2/mingw_64/lib");
        println!("cargo:rustc-link-lib=Qt6Core");
        println!("cargo:rustc-link-lib=Qt6Gui");
        println!("cargo:rustc-link-lib=Qt6Qml");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-arg=-lQt6Qml");
        println!("cargo:rustc-link-arg=-lQt6Gui");
        println!("cargo:rustc-link-arg=-lQt6Core");
        println!("cargo:rustc-link-arg=-lstdc++");
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }
}
