pub fn run(){
    let x=3;
    {
        let y=10;
        println!("x is {} and y is {}",x,y );
    }
    //the line below gives error so commented out
    // println!("y is {}",y );
    println!("x is {}",x );
    //function cannot take values out of scope 
    //so we need to use closures
    pub fn check(){
        //the line below gives x not in scope. 
        // println!("x is  {}",x );
        let x=22;
        println!("x is  {}",x );
    }
    //closures 
    let closure = ||{
        println!(" From Closure x is {}",x );
    };
    closure();
}