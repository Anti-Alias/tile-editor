use egui::{Widget, Ui, Response};

pub struct Editor {
    pub name: String,
    pub content: String
}

impl Editor {
    pub fn new(name: &str, content: &str) -> Editor {
        Editor {
            name: name.to_owned(),
            content: content.to_owned()
        }
    }
    pub fn ui(&self, ui: &mut Ui) -> Response {
        ui.label(self.content.as_str())
    }
}