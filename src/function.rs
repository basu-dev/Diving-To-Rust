pub fn run(){
    let mut a=3;
    let mut b=5;
    let z=10;
    //normal function
    fn sum(a:u32,b:u32){
        println!("{}",a+b );
    }
    // closure
    let multiple=|a:u32,b:u32|{println!("{}",a*b*z )};
    multiple(2,3);
    sum(a,z);
}
