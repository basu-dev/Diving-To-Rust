pub fn run(){
	//for primitive values
	let a=32;
	let b=a;
	println!(" a is {} and b is {}",a,b );
	//for non primitive values like string and vectors
	let strA="basu";
	let strB=strA;
	println!("strA is {} and strB is {}",strA,strB);

	// for vectors
	let vec_a=vec![32,34,53];

	let vec_b=&vec_a;
	println!("vecb:{:?}  veca: {:?}",vec_b,vec_a );
	let x=&vec_b;
	
	println!("{:?}",x );
}