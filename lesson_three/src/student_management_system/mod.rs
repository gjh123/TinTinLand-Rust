mod class;
mod course;
mod student;

pub use class::Class;
pub use course::Course;
pub use student::Student;

pub struct StudentManagementSystem {
    students: Vec<Student>,
    classes: Vec<Class>,
    courses: Vec<Course>,
}

impl StudentManagementSystem {
    // 创建一个新的学生管理系统
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
            classes: Vec::new(),
            courses: Vec::new(),
        }
    }

    // 添加学生
    pub fn add_student(&mut self, student: Student) {
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
    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students
            .iter()
            .find(|student| student.id == student_id)
    }

    // 更新学生信息
    pub fn update_student(&mut self, student_id: u32, updated_student: Student) -> bool {
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
    pub fn delete_student(&mut self, student_id: u32) -> bool {
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
