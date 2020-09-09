
pub fn run(){
    let  id:u8;
    id=8;
    println!("{}",id );
    //can push to string
    let mut name:String=String::from("BasuDev");
    name.push(' ');
    name.push_str("Adhikari");
    println!("My name is {}",name );
    //string of fixed length

    let mut myname:&str="Dev";
    myname="Cattt";
    println!("{} has length {}",myname ,myname.len());

    //tuples

    let tub:(&str,&str,u8)=("BasuDev","Kathmandu",23);
    println!(" {} is {} and lives in {}",tub.0,tub.2,tub.1);

    //array
    //array is list of items of same type and constant length ; length cannot be varied but array members can be mutated
    let mut arr=[1,3,5];
    println!("Array {:?} has length {}",arr,arr.len() );
    




}