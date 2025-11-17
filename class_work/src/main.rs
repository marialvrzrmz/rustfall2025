// fn pattern_match_simple() {
//     let by3 = num%3 == 0;
//     let by5 = num%5 == 0;

//     let result == match(by3, by5){
//         (true, true) => "FizzBuzz".to_string(),
//         (true, false) => "Fizz".to_string(),
//         (false, true) => "Buzz".to_string(),
//         (false, false) => num.to_string(),
//     };
//     result
// }

// fn main(){
    
//     let s = "hello"; // s is valid from this point forward
//         let s = String::from("Hello"); // Allocates memory on the heap
//     println!("message from heap: {}", s);

//     let mut s = 1234.to_string(); // Note: rules regarding mutability still apply
//     println!("message from heap: {}", s);

//     // Strings are mutable
//     s.push_str("4567");
//     println!("My string number: {}", s);
// }

// fn append_region(word: &mut String){
//     word.push_str("RGV");
// }

// fn borrow_ref_to_values() {
//     let mut x = "UT".to_string();
//     append_region(&mut x);
//     // let y = &x; // Borrowing a reference to 'x'
//     println!("{}", x); // Prints value to which y points, dereference happens implicitly
//     // println!("{:p} == {:p}", y, &x); // y and x have exactly the same address
// }

// struct Car {
//     seats: u8
//     model: String, 
// }

 //methods are added by IMPL statement
//  impl Car{
//      fn new(s:u8, m:String) -> Car{ //static method
//          Self {
//              seats = s;
//              model: m;

//          }
//      }
//     fn get_model(&self) -> &String{
//         return &self.model;
//     }
//     fn set_model(&mut self, new_model: String){
//         self.model = new_model;
//     }
//  }
 

//  fn main(){
//      let my_car = Car::new(4, "Tacoma".to_string());
//      println!("Number of seats: {}", my_car.seats);
//      println!("Car model: {}", my_car.model);
//  }

//  struct Student{
//     name: String,
//      major: String,
//  }

//  impl Student{
//     fn new(n: String, m: String) ->Student{
//         Self{
//             name: n,
//             major: m,
//         }
//     }
//     fn get_major(&self) -> &String{
//         return &self.major;
//     }
//     fn set_major(&mut self, new_major: String){
//         self.major = new_major;
//     }
//  }

//  fn main(){
//     let mut student_info = Student::new("Maria".to_string(), "Computer Science".to_string());
//     println!("Name: {}", student_info.name);
//     println!("Major: {}", student_info.get_major());
//     student_info.set_major("Spanish".to_string());
//     println!("Major: {}", student_info.get_major());
//  }

// use std::arch::asm;

// fn main() {
//     let message = b"Maria Alvarez\n";

//     unsafe {
//         // write syscall
//         asm!(
//             "mov rax, 1",  // syscall number for write
//             "mov rdi, 1",  // file descriptor: 1 is stdout
//             "syscall",
//             in("rsi") message.as_ptr(),
//             in("rdx") message.len(),
//             out("rax") _,
//             out("rcx") _,
//             out("r11") _,
//             clobber_abi("system")
//         );

//         // exit syscall
//         asm!(
//             "mov rax, 60", // syscall number for exit
//             "xor rdi, rdi", // status code 0
//             "syscall",
//             options(noreturn)
//         );
//     }
// }

// use std::fs::File;
// use std::io::Write;

// fn create_and_write_to_file() {
//     let mut file = File::create("example.txt").unwrap();
//     writeln!(file, "Hello, Rust file operations!").unwrap();
//     writeln!(file, "this is a new line").unwrap();
//     // writeln!(file, "Hello, Rust file operations!").unwrap();
//     // writeln!(file, "This is a new line.").unwrap();
// }

// fn main() {
//     create_and_write_to_file();
//     println!("File created and written successfully.");
// }

// use std::fs::File;
// use std::io::{Read, BufReader, BufRead};

// fn read_entire_file() {
//     let mut file = File::open("example.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     println!("File contents:\n{}", contents);
// }

// fn read_file_line_by_line() {
//     let file = File::open("example.txt").unwrap();
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         println!("{}", line.unwrap());
//     }
// }

// fn main() {
//     println!("Reading entire file:");
//     read_entire_file();

//     println!("\nReading file line by line:");
//     read_file_line_by_line();
// }

// use std::process::Command;

// fn executing_os_commands_linux() {
//     let output = Command::new("ls")
//         .arg("-l")
//         .output()
//         .expect("Failed to execute command");

//     println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
// }

// fn create_and_write_to_file() {
//     let mut file = File::create("example.txt").unwrap();
//     writeln!(file, "Hello, Rust file operations!").unwrap();
//     writeln!(file, "This is a new line.").unwrap();
// }

// fn read_entire_file() {
//     let mut file = File::open("example.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     println!("File contents:\n{}", contents);
// }

// fn main(){
//     executing_os_commands_linux();
// }

// #[derive(PartialEq, Debug)]
// enum Fruit {
//     Apple(String),
//     Banana(String),
//     Tomato(String),
// }

// struct Inventory {
//     fruit: Vec<Fruit>,
// }

// impl Inventory {
//     fn available_fruits(&self) { 
//         for f in &self.fruit{
//             print!("{:?}: ", f);
//             Self::tell_me_joke(f);
//         } 
//     }

//     fn tell_me_joke(fruit: &Fruit) { 
//         match fruit{
//             Fruit::Apple(_) => println!("You're the apple of my eye!"), 
//             Fruit::Banana(_) => println!("A banana a day keeps the grump away!"), 
//             Fruit::Tomato(_) => println!("Don't be a saucy tomato."),
//         }
//     }
// }

// fn main(){
//     let a = "An apple a day keeps the doctor away.".to_string();
//     let b = "A banana boosts energy in a peel.".to_string();
//     let t = "A tomato a day keeps the sunburn away.".to_string();

//     let fruits = vec![
//         Fruit::Banana(b),
//         Fruit::Apple(a),
//         Fruit::Tomato(t),
//     ];

//     let grocery_store = Inventory { fruit: fruits };

//     grocery_store.available_fruits();
// }


// fn using_function_as_variable() {
//     // Regular function
//     fn add(x: i32, y: i32) -> i32 {
//         x + y
//     }

//     // Function pointer
//     let f = add;
//     let result = f(1, 2);
//     println!("{}", result); 

//     // Closure with explicit types
//     let f = |x: i32, y: i32| { x + y };

//     // Simplified closure
//     let f = |x: i32, y: i32| x + y;

//     // Closure with inferred types
//     let f = |x, y| x + y;
    
//     let result = f(1, 2);
//     println!("{}", result);  // Output: 3
// }

// fn using_function_as_parameter() {
//     fn add(x: i32, y: i32) -> i32 {
//         x + y
//     }

//     fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
//         let result = operation(x, y);
//         println!("Result of operation: {}", result);    
//     }

//     calculator(1, 2, add);
//     calculator(1, 2, |x, y| x + y);
//     calculator(1, 2, |x, y| x - y);
//     calculator(1, 2, |x, y| x * y);
// }

fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    //inside of the struct create a field called name
    //and beside sounds your animal should print "hey my name is {}", name

    #[derive(Debug)]
    struct Dog{
        name: String,
    }
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            format!("Woof woof. My name is {}", self.name)
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new(); //<dyn Animal>
    
    zoo.push(Box::new(Dog{name: "Lucy".to_string()}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal.sound());
    }
}

fn main(){
    box_polymorphism();
}