pub struct Dog {
    pub name: String
}

impl Dog {
    pub fn new(name: &str) -> Dog {
        Dog {name: name.to_string()}
    }

    pub fn speak(&self) -> &str { 
        "WOOF!"
    }
}
