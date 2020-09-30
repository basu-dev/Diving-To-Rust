pub fn run(){
    let  a=2;
    let  b=1;
    let q=a/b;
    let r=a%b;
    let mut number=1;
     number=(a *(number% q))-
                    (r*(number/q));
    println!("{}",number)
}