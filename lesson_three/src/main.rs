mod student_management_system;

use student_management_system::{Class, Club, Course, Student, StudentManagementSystem as SMS};

fn main() {
    // Init StudentManagementSystem
    let mut sms = SMS::new();

    // ===== Student ====
    // 添加学生
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

    // 更新学生信息
    let updated_student = Student {
        id: 1,
        name: String::from("Elon Musk2"),
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

    // 删除学生信息
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

    // =====  Club =====
    // 创建社团
    sms.create_club(1, String::from("Programming Club"));

    // 添加学生到社团
    sms.add_student_to_club(1, 1);

    // 查询社团成员
    if let Some(members) = sms.get_club_members(1) {
        for member in members {
            println!("Club Member: {}", member);
        }
    }
}
