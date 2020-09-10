pub fn run(){
    let a=20;
    //if/else
    if a<10 {
         println!("{} is lesser than 10", a);
        }
    else {
        println!("{} is greater than 10", a);
    }
    let mut i=0;
    loop {
        println!("From loop if loop {}", i);
        i+=1;
        if i>9 { break;}
    }
    let mut j=0;
    while j<9{
        println!("From while loop {}",j );
        j+=1;
    }
    let  name= "Basu";
    enum Coins{
        Penny,
        Nickel,
        Dime,
        Quarter
    }
    fn check(coin:Coins){
        match coin {
            Coins::Penny=>println!("{}","It is penny" ),
            Coins::Nickel=>println!("{}","It is Nickel" ),
            Coins::Dime=>println!("{}","It is Dime" ),
            Coins::Quarter=>println!("{}","It is Quarter" )
         
        }
    }
    fn checkStr(st:&str){
        match st{
            "Basu"=>println!("{}","Basu" ),
            "Dev"=>println!("{}","Dev" ),
            _=>println!("{}","not Basu or Dev" )
        }
    }
    check(Coins::Penny);
    checkStr("Basu");
}