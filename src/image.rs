
use std::fs::File;
use std::path::Path;

pub fn run(){

    let path=Path::new("basua.png");
    let mut file=match File::open(&path){
        Err(_)=> panic!("Error"),
        Ok(e)=>println!("{:#?}",e)
    };
    // println!("{:#?}",file)
    let mut content=String::new();
	match file.read_to_string(&mut content){
		Err(e) => panic!("{:?}",e ),
		Ok(result) => println!("{:?}", content )
	}
}
