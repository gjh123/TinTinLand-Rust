mod class;
mod club;
mod course;
mod student;

pub use class::Class;
pub use club::Club;
pub use course::Course;
use std::collections::HashMap;
pub use student::Student;

pub struct StudentManagementSystem {
    students: Vec<Student>,
    classes: Vec<Class>,
    courses: Vec<Course>,
    clubs: HashMap<u32, Club>,
}

impl StudentManagementSystem {
    // 创建一个新的学生管理系统
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
            classes: Vec::new(),
            courses: Vec::new(),
            clubs: HashMap::new(),
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

    // 创建社团
    pub fn create_club(&mut self, id: u32, name: String) {
        let club = Club {
            id,
            name,
            members: Vec::new(),
        };
        self.clubs.insert(id, club);
    }

    // 添加学生到社团
    pub fn add_student_to_club(&mut self, club_id: u32, student_id: u32) -> bool {
        if let Some(club) = self.clubs.get_mut(&club_id) {
            club.members.push(student_id);
            true
        } else {
            false
        }
    }

    // 查询社团成员
    pub fn get_club_members(&self, club_id: u32) -> Option<&Vec<u32>> {
        self.clubs.get(&club_id).map(|club| &club.members)
    }
}
