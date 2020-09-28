use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
static TEXT: &str = "console.log('abc')";
pub fn run(){
	let path=Path::new("src/file.js");
	// println!("{:?}",path );

	let display=path.display();
	//create file
	let mut file=match File::create(&path){
		Err(e) => panic!(" \n\nSome Error to open {} , \n\n{:?}",display,e),
		Ok(result) =>{println!("{:?}",result );
				result
	}
};
	//write to file
	match file.write_all(TEXT.as_bytes()){
		Err(e) => panic!("\n\n{:?}",e ),
		Ok(result) => println!("{:?}", result)
	};
	//open file
	let mut file= match File::open(&path) {
		 Err(e) => panic!("{:?}",e ),
		 Ok(result) => result	

	};
	//read from file
	let mut content=String::new();
	match file.read_to_string(&mut content){
		Err(e) => panic!("{:?}",e ),
		Ok(result) => println!("{:?}", content )
	}
}