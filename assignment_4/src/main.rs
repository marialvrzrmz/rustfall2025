struct Student{
    name: String,
     major: String,
 }

 impl Student{
    fn new(n: String, m: String) ->Student{
        Self{
            name: n,
            major: m,
        }
    }
    fn get_major(&self) -> &String{
        return &self.major;
    }
    fn set_major(&mut self, new_major: String){
        self.major = new_major;
    }
 }

 fn main(){
    let mut student_info = Student::new("Maria".to_string(), "Computer Science".to_string());
    println!("Name: {}", student_info.name);
    println!("Major: {}", student_info.get_major());
    student_info.set_major("Spanish".to_string());
    println!("Major: {}", student_info.get_major());
 }