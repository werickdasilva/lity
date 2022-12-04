use lity::{application, Point, Size, Window, events::WindowEvent};

#[test]
pub fn show_window() {
    application(Window::new())
}

#[test]
pub fn title_window() {
    let window = Window::new().set_title(String::from("Title Window"));
    application(window)
}

#[test]
fn size_window() {
    use lity::{application, kurbo::Size, Window};

    let window = Window::new().set_size(Size::new(700., 300.));

    application(window)
}
#[test]
fn position_window() {
    let window = Window::new().set_positio(Point::new(400., 300.));

    application(window)
}
#[test]
fn resizable_window() {
    let window = Window::new().set_resizable(true);
    application(window)
}

#[test]
fn show_config_window() {
    let window = Window::new()
        .set_title("Show Config Window".to_string())
        .set_size(Size::new(200., 200.))
        .set_resizable(false)
        .set_positio(Point::new(0., 0.));

    application(window)
}

#[test]
fn event_close() {
    let window = Window::new().set_on_close(|e| {
        println!("Close Window....");
        *e = WindowEvent::OnExit;
    });

    application(window)
}
