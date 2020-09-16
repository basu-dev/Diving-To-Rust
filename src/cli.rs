use std::io;
use std::env::*;
use rand::Rng;
use std::cmp::Ordering;
pub fn run(){
   let args:Vec<String>=args().collect();
   println!("{:?}",args );
    println!("{}","Guessing Game");
    println!("{}","Take a guess" );
    let mut guess=String::new();
    let random_number=rand::thread_rng().gen_range(1,100);
    io::stdin().read_line(&mut guess).expect("There was some error");
    fn guess_result(guess:u8,random_number:u8){
        println!("{:?}",(guess,random_number) );
        let gs=&random_number;
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Increase"),
            Ordering::Greater => println!("Decrease"),
            Ordering::Equal=>println!("correct")
        }
    }
    let guess:u8=guess.trim().parse().expect("It is not a number");
    guess_result(guess,random_number);
 }