use crate::student::Student;

pub struct Course {
    pub id: u32,
    pub name: String,
    pub students: Vec<Student>,
}

impl Course {
    pub fn create(id: u32, name: String) -> Course {
        Course { id, name, students: Vec::new() }
    }

    pub fn read(&self) {
        println!("ID: {}, Name: {}", self.id, self.name);
        println!("Students:");
        for student in &self.students {
            student.read();
        }
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
    }

    pub fn delete(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.students.clear();
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
}
