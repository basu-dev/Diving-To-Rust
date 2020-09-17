use std::fs::File;
use std::path::Path;
use std::error::Error;
pub fn run(){
	let path=Path::new("quiz.rs");
	// println!("{:?}",path );
	let display=path.display();
	let mut file=match File::open(&path){
		Err(e) => panic!(" Not opening {} , {:?}",display,e),
		Ok(result) =>println!("{:?}",result )
	};
	println!("{:?}",file );
}