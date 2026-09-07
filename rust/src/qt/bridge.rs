#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qproperty(bool, expanded)]
        #[qproperty(QString, current_edge)]
        #[qproperty(QString, primary_monitor_name)]
        #[qproperty(f64, scale_factor)]
        #[qproperty(QString, summary_text)]
        type CodeHaloBridge = super::CodeHaloBridgeRust;

        #[qinvokable]
        fn toggle_expanded(self: Pin<&mut CodeHaloBridge>);

        #[qinvokable]
        fn set_edge(self: Pin<&mut CodeHaloBridge>, edge: &QString);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct CodeHaloBridgeRust {
    pub expanded: bool,
    pub current_edge: QString,
    pub primary_monitor_name: QString,
    pub scale_factor: f64,
    pub summary_text: QString,
}

impl qobject::CodeHaloBridge {
    pub fn toggle_expanded(mut self: Pin<&mut Self>) {
        let next = !self.expanded();
        self.as_mut().set_expanded(next);
    }

    pub fn set_edge(mut self: Pin<&mut Self>, edge: &QString) {
        self.as_mut().set_current_edge(edge.clone());
    }
}
