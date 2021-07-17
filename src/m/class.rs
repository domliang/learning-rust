#[derive(Debug)]
pub struct ClassName {
    field: i32,
    r: String,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName {
            field: value,
            r: String::from("r"),
        }
    }

    pub fn print(&self) {
        println!("the number is {}", self.field);
    }
}

impl ClassName {
    pub fn hi(&mut self, s: String) {
        self.r = s.clone();
        print!("hi {} {}", self.field, self.r);
    }
}
