pub fn run(){
    let  a:f64=1.777773823;
    let  b:f64=3.83205809342;
    let c:f64=4.23423452342;
    let d = b % a;
    let e = c*b*d;
    let f = b%a;
    let mut x:u64=10000000000;
    let each = x/20;
    let mut z:u64 = 0;
    let mut reset_val = 0;
    let mut percentage = 0;
    while x>0{
        if reset_val==each{
            reset_val=0;
            percentage+=5;
            println!("{}% completed!",percentage );
        }
        reset_val+=1;
        z+=1;
        x-=1;
    }
    println!("z is {}",z );

}