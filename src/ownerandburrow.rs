pub fn run(){
    //Case 1. values to variables are fixed size and stored in stack.
    //Its valid
    let a=3;
    let b=a;
    println!("a: {},b: {}",a,b);


    let mut x=3;
    let y=x;
    x=5;
    println!("x:{} , y:{}",x,y);
    //Case 2:For string literals or str, It is valid as it is stored in program bytes but not on heap
    let strr="abc";
    let new_strr=strr;
    println!("strr:{} , new_strr:{}",strr,new_strr);

    //Case 3:

    let string=String::from("Nice to meet you");
    //Here we need to use &sign to make it reference
    let new_string=&string;
    println!("string: {}, new_string: {}",string,new_string);

}