
pub fn run(){
    #[derive(Debug)]
    enum Position{
        Student,
        Teacher,
        Principal
    }

    #[derive(Debug)]
    struct Person{
        position:Position,
        name:String,
        age:u8

    }
    impl Person{
        fn new(position:Position,name:String,age:u8)->Person{
            Person{position,name,age}
        }
    }
    let me=Person::new(Position::Student,String::from("Basu Dev"),32);
    println!("{:?}",me);
//std library Option enum preluded with types Some and None so no need to do Option::Some or Option::None ,directly Some or None
    let x:Option<i32>=Some(3);
    //or directly x=Some(3)
    println!("{:#?}",x );
    let mut none: Option<i32>=None;
    none=Some(3);
    println!("{:?}",none );
//using match
    fn add_one(x:Option<i32>)->Option<i32>{
        match x{
            None=>None,
            Some(x)=>Some(x+1)
        }
    }
//using if let
    // fn add_five(x:Option<i32>)->Option<i32>{
    //     if let Some(3)=x{
    //        println!("{}","hi" );
    //        Some(x+5)
    //     }
    //     else{
    //         None
    //     }
    // }
    // Sorry I am still confused with if let
    let y=add_one(x);
    println!("{:?}",y );
    println!("{:?}",add_one(None) );
    // println!("{:?}",add_five(Some(3)) );
    
}