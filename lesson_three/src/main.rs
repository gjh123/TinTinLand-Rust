struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: u32,
}

struct Class {
    id: u32,
    name: String,
}

struct Course {
    id: u32,
    name: String,
}

// 定义学生管理系统结构体
struct StudentManagementSystem {
    students: Vec<Student>,
    classes: Vec<Class>,
    courses: Vec<Course>,
}

impl StudentManagementSystem {
    // 创建一个新的学生管理系统
    fn new() -> Self {
        Self {
            students: Vec::new(),
            classes: Vec::new(),
            courses: Vec::new(),
        }
    }

    // 添加学生
    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    // 添加班级
    fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    // 添加课程
    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    // 根据学生ID获取学生信息
    fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students
            .iter()
            .find(|student| student.id == student_id)
    }

    // 更新学生信息
    fn update_student(&mut self, student_id: u32, updated_student: Student) -> bool {
        if let Some(student) = self
            .students
            .iter_mut()
            .find(|student| student.id == student_id)
        {
            *student = updated_student;
            true
        } else {
            false
        }
    }

    // 删除学生
    fn delete_student(&mut self, student_id: u32) -> bool {
        if let Some(index) = self
            .students
            .iter()
            .position(|student| student.id == student_id)
        {
            self.students.remove(index);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut sms = StudentManagementSystem::new();

    let student1 = Student {
        id: 1,
        name: String::from("Elon Musk"),
        age: 18,
        class_id: 1,
    };

    let student2 = Student {
        id: 2,
        name: String::from("马云"),
        age: 19,
        class_id: 2,
    };

    sms.add_student(student1);
    sms.add_student(student2);

    let student = sms.get_student(1);
    if let Some(student) = student {
        println!(
            "Student found: {} (ID: {}, Age：{})",
            student.name, student.id, student.age
        );
    } else {
        println!("Student not found");
    }

    let updated_student = Student {
        id: 1,
        name: String::from("Elon Musk2 --updated"),
        age: 30,
        class_id: 2,
    };
    sms.update_student(1, updated_student);

    let student = sms.get_student(1);
    if let Some(student) = student {
        println!(
            "Updated Student: {} (ID: {}, Age：{})",
            student.name, student.id, student.age
        );
    } else {
        println!("Student not found");
    }

    if let true = sms.delete_student(2) {
        println!("Deleted Student：success");
    } else {
        println!("Deleted Student：failed");
    }

    let student = sms.get_student(2);
    if let Some(student) = student {
        println!("Student found: {} (ID: {})", student.name, student.id);
    } else {
        println!("ID：1 Student not found");
    }
}
