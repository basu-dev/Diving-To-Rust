pub fn print(){
    println!("abc");

    println!("My name is {}","Basu");

    println!("{} is {}","Bob",25);

    println!("{0} is {1} . {0} is a student.","Dev",22);

    println!("Binary: {:b}, Octal is :{:o}",10,10);

    println!("My name is {name}. I am {age} years old.",name="Basu",age=32 );
    // debug traits
    #[derive(Debug)]
    struct Person{
    	name:String,
    	age:u8
    }
    let person=Person{
    	name:"Dev".to_string(),
    	age:32
    };
    println!("{:#?}",person );
    println!("Data is{:?}",("Nice",29,true))
}