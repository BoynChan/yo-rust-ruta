mod gui_library;
mod health_statistic;
mod library;
mod luhn_alogorithm;
mod polygon;

use gui_library::{Button, Label, Widget, Window};
fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
