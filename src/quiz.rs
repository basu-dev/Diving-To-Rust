use rand::Rng;
use std::io;
pub fn run(){
	#[derive(Debug,Clone)]
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
	};

	let questions:Vec<Question>=vec![
		Question::new(
					"What is the capital city of Nepal?",
					["Pokhara","Kathmandu","Dharan","Lalitpur"],
					"Kathmandu"
		),
		Question::new(
					"Who is the first Prime Minister of Nepal ?",
					["Gagan Thapa","Matrika Koirala","Bhimsen Thapa","Jung Bahadur Rana"],
					"Bhimsen Thapa"
		),
		Question::new(
					"Who is the captain of Nepali Cricket Team?",
					["Gyanendra Malla","Paras Khadka","Sagar Thapa","Sandeep Lamichhane"],
					"Gyanendra Malla"
		),
		Question::new(
					"Who is the current President of USA?",
					["Barack Obama","Donald Trumph","John F. Kennedy","Vladimir Putin"],
					"Donald Trumph"
			),
		Question::new(
					"What is the area of Nepal in kilometer square?",
					["1,47,181","2,57,181","1,78,181","2,54,541"],
					"1,47,181"
			),


	];
	let mut questions_copy:Vec<Question>=questions.clone();
	println!("\n================ Quiz Game =============\n");
	static mut NUM:i8=4;
	static mut SCORE: u8 = 0;
	fn quiz(questions:Vec<Question>){
		unsafe {NUM-=1};
		let mut terminate=false;
		 unsafe {if NUM <1 {
		 	terminate=true;
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
			unsafe {SCORE+=1}
		}
		if terminate{
			println!("\n=============== Game Over ===============\n" );
			unsafe{println!("              YOUR SCORE IS {} / 4", SCORE );};
			// println!("\n======== The Correct Answers Are ========\n");
		}
		else {
			quiz(questions)
		}
	}
	quiz(questions_copy);
}