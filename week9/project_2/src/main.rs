use std::io::Write;

fn main() {
    //Student name, Matric number, Department,Level
let student_vec: Vec<(&str, &str, &str, u32)> = vec![
    ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
    ("Adams Aliyu", "ECO10110101", "Economics", 100),
    ("Sharina Bolade", "CSC10320020", "Computer", 300),
    ("Adekunle Gold", "EEE11020202", "Electrical", 200),
    ("Blanca Edemoh", "MEE10202001", "Mechanical", 100), 
 ];
 
 let mut content = String::from("Student name, Matric number, Department, Level");
 
 for student in student_vec.iter() {
     let line = format!("{}\n,{}\n,{}\n,{}\n", student.0, student.1, student.2, student.3);
     content.push_str(&line);
 
 }
 
 println!("PAU SMIS");
 println!("{}", content);
 let mut file = std::fs::File::create("data.csv").expect("create failed");
 file.write_all(content.as_bytes()).unwrap();
}

