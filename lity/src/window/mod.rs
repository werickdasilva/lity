pub(crate) mod window_connector;

pub struct Window {
    pub title: String,
    pub size: crate::Size,
    pub resizable: bool,
    pub position: crate::Point,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "Lity".to_string(),
            size: crate::Size::new(600., 400.),
            resizable: true,
            position: crate::Point::new(100., 100.)
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
}