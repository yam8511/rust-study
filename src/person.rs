use std::fmt::Display;

// pub = publish
#[derive(Debug)] // <--- 替物件加上Debug屬性 方可 "{:?}"
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    } // static function

    pub fn say_hi(&self) -> String {
        println!("I'm {}", self.name);
        format!("I'm {}", self.name)
    } // dynamic function

    pub fn change_name(&mut self, name: String) {
        self.name = name
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person({}, {})", self.name, self.age)
    }
}
