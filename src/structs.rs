
pub fn run(){
struct Person<'a>{
    name:&'a str,
    address:&'a str,
    age:u8
};
let a= Person{
    age:34,
    address:"Kapan",
    name:"Basu"
};
let b=Person("Cat","Catmandu",23);
// println!("age: {}, name:{} , address:{}", a.age,a.name,a.address);
println!("age: {}, name:{} , address:{}", b.age,b.name,b.address);
}