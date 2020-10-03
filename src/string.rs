//&str is only string type in core library which is reference to string slices
//String type is provided by the standard library which is owned type
pub fn run(){
    let a:&str="basu";
    //any type that implements display trait can use to_stirng()method to convert to String type.
    let string:String=String::from(a);
    //or
    let mut string=a.to_string();
    string.push(' ');
    string.push_str("dev ");
    let stringb=string+&"adhikari";
    println!("{} {}",a,stringb );

}