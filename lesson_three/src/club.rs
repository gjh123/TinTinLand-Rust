use crate::student::Student;

pub struct Club {
    pub id: u32,
    pub name: String,
    pub members: Vec<Student>,
}

impl Club {
    pub fn create(id: u32, name: String) -> Club {
        Club { id, name, members: Vec::new() }
    }

    pub fn read(&self) {
        println!("ID: {}, Name: {}", self.id, self.name);
        println!("Members:");
        for member in &self.members {
            member.read();
        }
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
    }

    pub fn delete(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.members.clear();
    }


    pub fn add_member(&mut self, student: Student) {
        self.members.push(student);
    }
}
