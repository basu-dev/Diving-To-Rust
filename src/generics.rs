use std::cmp::PartialOrd;

pub fn run(){
//    fn largest<T>(list:&[T])->T{
//     let mut largest=list[0];
//     for &item in list{
//         if item > largest{
//             largest = item
//         }
//     }
//     largest
//    }
#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T
}
let a = Point{x:1,y:3};
let b = Point{x:"abc".to_string(),y:"xyx".to_string()};
println!("a is {:#?} and b is {:#?}",a,b);

#[derive(Debug)]
struct Shape<T,U>{
    name:String,
    length:T,
    breadth:U
}

let rectangle = Shape{
    name:"Rectangle".to_string(),
    length: 14.43,
    breadth:10.44
};
let triangle = Shape{
    name:"triangle".to_string(),
    length:32,
    breadth:33 
};
println!("rectangle: {:#?}, triangle {:#?}",rectangle,triangle);

impl<T,U> Shape<T,U>{
    fn mixup<V,W>(self,other: Shape<V,W>)->Shape<T,W>{
        Shape{
            name:"New Shape".to_string(),
            length:self.length,
            breadth:other.breadth
        }
    }
}
let new = triangle.mixup(rectangle);
println!("new Shape {:#?}",new );
}