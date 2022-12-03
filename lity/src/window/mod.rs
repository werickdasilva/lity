use self::events::WindowEvent;

pub(crate) mod window_connector;
pub mod events;


pub struct Window {
    pub title: String,
    pub size: crate::Size,
    pub resizable: bool,
    pub position: crate::Point,
    pub show_title_bar: bool,
    pub on_close: Option<fn(&mut WindowEvent)>
}

impl Window {   
    pub fn new() -> Self {
        Self {
            title: "Lity".to_string(),
            size: crate::Size::new(600., 400.),
            resizable: true,
            position: crate::Point::new(100., 100.),
            show_title_bar: true,
            on_close: None,
        }
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_size(mut self, size: crate::Size) -> Self {
        self.size = size;
        self
    }

    pub fn set_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    
    pub fn set_positio(mut self, position: crate::Point) -> Self {
        self.position = position;
        self
    }

    pub fn set_title_bar(mut self, visible: bool) -> Self {
        self.show_title_bar = visible;
        self
    }

    pub fn set_on_close(mut self, func: fn(&mut WindowEvent)) -> Self {
        self.on_close = Some(func);
        self
    }


}