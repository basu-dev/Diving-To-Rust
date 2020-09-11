use std::io;
pub fn run(){
    let number=206;
    println!("{}","Guessing Game" );
    println!("{}","Take a guess" );
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("There was some error");
    fn guess_result(guess:u8){
        match guess {
            32=>println!("Guess Right{}", guess),
            _=>println!("Wrong guess {}",guess )
        };
    }
    guess_result(&guess);
    println!("You guessed {}",guess );


}