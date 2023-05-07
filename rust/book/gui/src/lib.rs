pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw button
    }
}

impl Screen {
    pub fun run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}