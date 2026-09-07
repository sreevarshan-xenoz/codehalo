#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct CodeHaloBridge {
        #[qproperty]
        expanded: bool,
        #[qproperty]
        current_edge: QString,
        #[qproperty]
        primary_monitor_name: QString,
        #[qproperty]
        scale_factor: f64,
        #[qproperty]
        summary_text: QString,
    }

    impl cxx_qt::Threading for CodeHaloBridge {}

    impl ffi::CodeHaloBridge {
        #[qinvokable]
        pub fn toggle_expanded(self: core::pin::Pin<&mut Self>) {
            let next = !self.expanded();
            self.set_expanded(next);
        }

        #[qinvokable]
        pub fn set_edge(mut self: core::pin::Pin<&mut Self>, edge: &QString) {
            self.as_mut().set_current_edge(edge.clone());
        }

        #[qinvokable]
        pub fn refresh_usage(self: core::pin::Pin<&mut Self>) {
            // Signal to async tokio worker to trigger provider updates
        }
    }
}
