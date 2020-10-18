use core::fmt::Display;

pub fn run(){
    trait Summery{
         fn summerize(&self){
             println!("This issummery")
         }
    }
    struct Point<T,U>{
       pub x:T,
        pub y:U
    }
    impl<T:Display,U:Display> Summery for Point<T,U>{
        fn summerize(&self){
            println!("x: {} & y: {}",self.x,self.y );
        }
    }
    let a =Point{
        x:32,
        y:44
    };
    a.summerize();
//The below example uses copy trait and partialord trait to function
//Copy trait is to copy items from stach and partialrod trait for general comparisions
    pub fn largest<T:Copy + PartialOrd>(list:&[T])->T{
        let mut largest=list[0];
        for &item in list{
            if largest < item{
                largest = item
            }
        };
         largest

    }
    let a = [4,3,5,10,2];
    println!("{}",largest(&a) );
}
