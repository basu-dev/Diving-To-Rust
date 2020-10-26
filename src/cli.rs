use std::io;
use std::env::*;
use rand::Rng;
use std::cmp::Ordering;
pub fn run(){
    println!("{}","Guessing Game");
    println!("{}","Take a guess" );
    let mut guess=String::new();
    let random_number=rand::thread_rng().gen_range(1,100);
    take_guess(random_number.to_string());
    let guess:u8=guess.trim().parse().expect("It is not a number");
}
fn guess_result(guess:&str,random_number:&str){
    println!("{:?}",(guess,random_number));
    match guess.cmp(&random_number) {
        Ordering::Less => {println!("Increase")
                            
    },
        Ordering::Greater => println!("Decrease"),
        Ordering::Equal=>println!("correct")
    }
}
 fn take_guess(random_number:String){
     let mut guess = String::new();
     io::stdin().read_line(&mut guess).is_ok();
     guess_result(&guess, &random_number);
     
 }