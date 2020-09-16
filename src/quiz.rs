use rand::Rng;
use std::io;
pub fn run(){
	#[derive(Debug)]
	struct Question<'a>{
		question:&'a str,
		options:[&'a str;4],
		answer:&'a str
	}
	impl Question <'static>{
		fn new(quest:&'static str,opts:[&'static str;4],ans:&'static str)->Question<'static>{
			Question{
				question:quest,
				options:opts,
				answer:ans
			}
		}
		//  fn options_list(&'static self){
		// 	println!("{:?}","asdf" );
		// }
	};

	let questions:Vec<Question>=vec![
		Question::new("What is the capital city of Nepal?",
					["Pokhara","Kathmandu","Dharan","Lalitpur"],
					"Kathmandu"
						),
		Question::new(
					"Who is the first Prime Minister of Nepal",
					["Gagan Thapa","Matrika Koirala","Bhimsen Thapa","Jung Bahadur Rana"],
					"Bhimsen Thapa"
		),
		Question::new(
					"Who is the captain of Nepali Cricket Team?",
					["Gyanendra Malla","Paras Khadka","Sagar Thapa","Sandeep Lamichhane"],
					"Sandeep Lamichhane"
		)

	];
	println!("================ Quiz Game =============");
	static mut num:i8=4;
	static mut score: u8 = 0;
	fn quiz(questions:Vec<Question>){
		unsafe {num-=1};
		let mut terminate=false;
		 unsafe {if num <1 {
		 	terminate=true;
			// println!("{:?}","The score is 4" )
		}
	}
		let mut input=String::new();
		let index=rand::thread_rng()
		.gen_range(0,questions.len()) as usize;
		let question=&questions[index];
		println!("{}",questions[index].question);
		for i in 0..question.options.len() as usize{
			println!(" {}: {}",i+1,question.options[i]);
		}
		std::io::stdin()
			.read_line(&mut input)
			.expect("Invalid Input");
		let input:usize=input.trim().parse().expect("Not a number");
		if question.options[input - 1]==question.answer{
			unsafe {score+=1}
		}
		if terminate{
			// quiz(questions)
			println!("=============== Game Over ===============" );
			unsafe{println!("              The score is {}", score );};
			println!("=========The Correct Answers Are=========");


		}
		else {
			quiz(questions)
		}
	}
	quiz(questions);
}