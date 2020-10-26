// this module doesn't produce random numbers
// I created this to test the speed of cargo build vs cargo build --release and found the speed of release version almost 10 times faster.
pub fn run(){
    let mut x:u64=100000000000;
    let each = x/100;
    println!("each is {}",each );
    let mut z:u64 = 0;
    let mut reset_val = 0;
    let mut percentage = 0;
    while x>0{
        if reset_val==each{
            reset_val=0;
            percentage+=1;
            println!("{}% completed!",percentage );
        }
        reset_val+=1;
        z+=1;
        x-=1;
    }
    println!("z is {}",z );

}