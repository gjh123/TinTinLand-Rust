pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

impl Student {
    pub fn create(id: u32, name: String, age: u8) -> Student {
        Student { id, name, age }
    }

    pub fn read(&self) {
        println!("ID: {}, Name: {}, Age: {}", self.id, self.name, self.age);
    }

    pub fn update(&mut self, name: String, age: u8) {
        self.name = name;
        self.age = age;
    }

    pub fn delete(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.age = 0;
    }
}
