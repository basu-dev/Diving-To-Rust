pub fn run(){
let vec = vec!["Basu","Dev","Adhikari"];
let first = &vec[0..vec.len()];

println!("{:?} \n{:?} \n{:?}",vec,first,vec.contains(&"Dev"));
}