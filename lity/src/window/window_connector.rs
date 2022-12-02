use druid_shell as shell;

pub(crate) struct WindowConnector;

impl WindowConnector {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl shell::WinHandler for WindowConnector {
    fn connect(&mut self, handle: &shell::WindowHandle) {}

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut shell::piet::Piet, invalid: &shell::Region) {}

    fn destroy(&mut self) {
        shell::Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
