pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    pub fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        return 0;
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).unwrap();
        buffer.write_str("\n").unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        return 0;
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer
            .write_str(format!("| {} |", self.label.label).as_str())
            .unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        return 0;
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str("=======\n").unwrap();
        buffer.write_str(&self.title).unwrap();
        buffer.write_str("\n").unwrap();
        buffer.write_str("=======\n").unwrap();
        buffer.write_str("\n").unwrap();
        for i in &self.widgets {
            i.draw_into(buffer);
            buffer.write_str("\n").unwrap()
        }
    }
}

pub fn print_gui() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
