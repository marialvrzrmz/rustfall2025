// Define a trait to represent behavior
trait ShowInfo {
    fn show_info(&self);
}

// Shared fields: GPA and Major
#[derive(Debug)]
struct Undergrad {
    name: String,
    major: String,
    gpa: f32,
}

#[derive(Debug)]
struct Grad {
    name: String,
    major: String,
    gpa: f32,
    thesis: String,
}

// Implement ShowInfo for both
impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad Student: {}\nMajor: {}\nGPA: {}\n",
            self.name, self.major, self.gpa
        );
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Graduate Student: {}\nMajor: {}\nGPA: {}\nThesis: {}\n",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}

// Enrollment struct using generics + trait bounds
struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn add_student(&mut self, student: T) {
        self.students.push(student);
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

// Example usage
fn main() {
    let u1 = Undergrad {
        name: "Maria".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.8,
    };

    let g1 = Grad {
        name: "Silvia".to_string(),
        major: "Computer Engineering".to_string(),
        gpa: 3.9,
        thesis: "AI in Robotics".to_string(),
    };

    // We can store each type of student separately
    let mut under_enrollment = Enrollment::<Undergrad>::new();
    under_enrollment.add_student(u1);
    
    let mut grad_enrollment = Enrollment::<Grad>::new();
    grad_enrollment.add_student(g1);

    println!("--- Undergrad Students ---");
    under_enrollment.show_all();

    println!("--- Graduate Students ---");
    grad_enrollment.show_all();
}
