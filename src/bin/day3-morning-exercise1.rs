// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer: String = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
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
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
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
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width: usize = self.width();

		writeln!(buffer, "+-{:-<1$}-+", "", width).unwrap();
		writeln!(buffer, "| {:^width$} |", self.label.label).unwrap();
		writeln!(buffer, "+-{:-<1$}-+", "", width).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
		std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
		let width: usize = self.width();

		let mut middle: String = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut middle);
        }

		writeln!(buffer, "+={:=<1$}=+", "", width).unwrap();
		writeln!(buffer, "| {:^width$} |", self.title).unwrap();
		writeln!(buffer, "+={:=<1$}=+", "", width).unwrap();
		for line in middle.lines() {
			writeln!(buffer, "| {:^width$} |", line).unwrap();
		}
		writeln!(buffer, "+={:=<1$}=+", "", width).unwrap();
    }
}

fn main() {
    let mut window: Window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}