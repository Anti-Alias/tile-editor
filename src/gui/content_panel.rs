use egui::{Widget, Ui, Response};

struct ContentPanel<'a> {
    pub text: &'a str
}

impl<'a> ContentPanel<'a> {
    fn new(text: &str) -> ContentPanel {
        ContentPanel { text }
    }
}

impl<'a> Widget for ContentPanel<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.text)
    }
}