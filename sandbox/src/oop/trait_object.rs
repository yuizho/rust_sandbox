trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Button: (label: {})", self.label);
    }
}

struct Text {
    value: String,
}
impl Draw for Text {
    fn draw(&self) {
        println!("Text: (value: {})", self.value);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Text {
                value: String::from("text value"),
            }),
            Box::new(Button {
                label: String::from("button1"),
            }),
        ],
    };
    screen.run();
}
