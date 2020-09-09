
pub fn run(){
    //integers are of many types like u8, u32 , u64, etc and same for i like i8 ....

    let  id:u8;
    id=8;
    //float
    let myfload:f64=1.34;
    println!("{} {}",id,myfload );
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
    arr[1]=7;
    println!("Array {:?} has length {}",arr,arr.len() );
    for i in arr.iter(){
        println!("{}",i );
    }
    //mutating on iterations
    for i in arr.iter_mut(){
        *i+=2;
        let abc=i.to_string();
        println!("{} and {}",i ,abc);
    }

    //vectors
    //vectors are arrays where datas can be pushed and popped

    let _vec:Vec<u8>=vec![1,2,3];
    let mut names:Vec<&str>=vec!["Basu","Rony","John"];
    let lst=vec!["Name"];
    let res=names.push("Jane");
    println!("{:?} , {:?} {:?}",_vec, names, res);


}