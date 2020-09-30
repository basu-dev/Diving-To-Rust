
pub fn run(){
//structs are similar to class in JavaScript or any other languages.
struct Person{
    name : String,
    address:String,
    age:u8
};
//named structs
let _a= Person{
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

    #[derive(Debug)]
    struct Rectangle{
        length:u8,
        breadth:u8
    }
    impl Rectangle{
        fn new(length:u8,breadth:u8)->Rectangle{
            Rectangle{
                length:length,
                breadth:breadth
            }
        }
        fn square(length:u8)->Rectangle{
            Rectangle{
                length,
                breadth:length
            }
        }
        fn area(&self)->u8{
            &self.length *&self.breadth
        }
    }

    let rect=Rectangle::new(32,32);
    println!("{:#?}",rect);
    let sqr=Rectangle::square(2);
    println!("{:#?},area: {}",sqr,sqr.area());
    
    #[derive(Debug)]
    struct Cube{
        length:u32,
        breadth:u32,
        height:u32
    }
    impl Cube{
        fn new(length:u32,breadth:u32,height:u32)->Cube{
            Cube{
                length,breadth,height
            }
        }
    }
    let cub=Cube::new(1,3,4);
    println!("{:#?}",cub)

}