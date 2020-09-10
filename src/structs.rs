
pub fn run(){
//structs are similar to class in JavaScript or any other languages.
struct Person{
    name : String,
    address:String,
    age:u8
};
//named structs
let a= Person{
    age:34,
    address:"Kapan".to_string(),
    name:"Basu".to_string()
};
//tuple structs
struct Product(String,u32);
let product=Product("Laptop".to_string(),65000);
println!("{} costs {}",product.0,product.1 );

//this is like prototypes in JavaScript or methods inside class
impl Person{
    fn new(name:&str,address:&str,age:u8)->Person{
        Person{
            name:name.to_string(),
            address:address.to_string(),
            age:age
        }
    }
    fn full_statement(&self){
        println!("{0} is {1}.{0} lives in {2}. ",&self.name,&self.age,&self.address );
    }
    fn update_age(&mut self, newage:u8){
        self.age=newage;
    }
}

//:: uses static methods 
// . uses object methods if these are correct naming conventions.
let mut b=Person::new("Cat","Catmandu",23);
b.update_age(32);
b.full_statement();
}