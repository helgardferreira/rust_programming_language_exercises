// defining a trait for common behaviour
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // components field holds a vector of trait objects that implement Draw
    /*
    A generic type parameter can only be substituted with one concrete type at a
    time, whereas trait objects allow for multiple concrete types to fill in for
    the trait object at runtime.
    */
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// using traits for polymorphic behaviour
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// implementing the trait


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
