use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

#[derive(Debug)]
struct Course {
    name: String,
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    //fn enroll(&mut self, student: &'a Student, course: &'a Course) {
    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

fn main() {
    let jhon = Student {
        name: "Jhon".into(),
    };

    let course = Course {
        name: "Intro to Rust".into(),
    };

    let mut p = Platform::new();
    p.enroll(&jhon, &course);

    for c in jhon.courses(p) {
        println!("Jhon is taking course = {}", c);
    }
}
