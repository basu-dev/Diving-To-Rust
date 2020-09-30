#[path = "./school/education.rs"]
mod education;
#[path = "./school/finance.rs"]
mod finance;
use education::Education::{ Grade,Student} ;
use finance::Finance::*;

pub fn run(){
    let students:Vec<&Student>;
    let employees:Vec<&Employee>;
    let second_std=Student::new(String::from("Basu Dev Adhikari"),Grade::One);
    let third_std=Student::new(String::from("Narayan Gopal"),Grade::Ten);
    let first_std=Student::new(String::from("Preeti Sapkota"),Grade::Four);
    students=vec![&first_std,&second_std,&third_std];
    let principal=Employee::new(String::from(" Pushpa Pradhan"),Position::Principal(3200));
    employees=vec![&principal];
    let a=Student::get_by_id(&students,second_std.id);

println!("{:#?}",a);

    // println!("{:#?}",&employees);

    // println!("{:#?}",&students);
}