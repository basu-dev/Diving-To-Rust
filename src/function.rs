pub fn run(){
    let a=3;
    let b=5;
    let z=10;
    //normal function
    fn sum(a:u32,b:u32)->u32{
        println!("{}",a+b );
        a+b
    }
    // closure
    let multiple=|a:u32,b:u32|{println!("{}",a*b*z )};
    multiple(2,3);
    let cat=sum(a,z);
    println!("{}",cat );
}
