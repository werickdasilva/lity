use druid_shell as shell;

use super::{events::WindowEvent};

pub(crate) struct WindowConnector {
    window_handler: shell::WindowHandle,
    on_close: Option<fn(&mut WindowEvent)>,
    window_event: WindowEvent,
}

impl WindowConnector {
    pub(crate) fn new(on_close: Option<fn(&mut WindowEvent)>) -> Self {
        Self {
            window_handler: Default::default(),
            on_close,
            window_event: WindowEvent::OnExit,
        }
    }
}

impl shell::WinHandler for WindowConnector {
    fn connect(&mut self, handle: &shell::WindowHandle) {
        self.window_handler = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut shell::piet::Piet, invalid: &shell::Region) {}

    fn request_close(&mut self) {
        if let Some(close) = self.on_close {
            (close)(&mut self.window_event);
        }

        match self.window_event {
            WindowEvent::OnExit => {
                self.window_handler.close();
            },
            WindowEvent::OnLeaveOpen => ()
        }
    }

    fn destroy(&mut self) {
        shell::Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
