mod window;
pub use window::*;
use druid_shell as shell;
pub use  shell::{kurbo::*, piet::*};

pub fn application(window: Window) {
    let app =  shell::Application::new().unwrap();
    let window_connector = Box::new(window_connector::WindowConnector::new());
    let mut window_build = shell::WindowBuilder::new(app.clone());
    window_build.set_title(window.title);
    window_build.set_size(window.size);
    window_build.resizable(window.resizable);
    window_build.set_handler(window_connector);
    let window_build = window_build.build().unwrap();

    window_build.show();

    app.run(None)
}