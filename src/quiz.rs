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
					["Barack Obama","Vladimir Putin","John F. Kennedy","Donald Trumph"],
					"Donald Trumph"
			),
		Question::new(
					"What is the area of Nepal in kilometer square?",
					["1,47,181","2,57,181","1,78,181","2,54,541"],
					"1,47,181"
			),
		Question::new(
					"Who won the UEFA Championns League 2020?",
					["Bayern Munchen","Paris Saint German","RB Leipeig","Barcelona"],
					"Bayern Munchen"
		),
		Question::new(
					"Who is the inventer of Facebook?",
					["Evan You","John Doe","Mark Zuckerberg","Bill Gates"],
					"Mark Zuckerberg"
			),
		Question::new(
					"'How you doin'?' catchphrase belongs to which character in famous TV Show 'Friends'?",
					["Joey Tribbiani","Phoebe Buffay","Chandler Bing","Rachel Green"],
					"Joey Tribbiani"
			),
		Question::new(
					"Who was the ruler of Germany during World War II?",
					["Nepolean Bonaparte","Winston Churchill","Joseph Stalin","Adolf Hitler"],
					"Adolf Hitler"
			),
		Question::new(
					"Who is the inventor of Telephone?",
					["Galileo Galille","Bill Gates","Graham Bell","Albert Einstein"],
					"Graham Bell"
			),
		Question::new(
					"Which band does the song 'Hey Jude!' belongs to?",
					["Pink Floyd","Eagles","Queen","The Beatles"],
					"The Beatles"
			)

	];
	println!("\n================ Quiz Mania =============");
	static mut NUM:i8=4;
	static mut SCORE: u8 = 0;
	let mut used_indices:Vec<String>=vec![];
	fn get_index(len:usize)->usize{
		let index=rand::thread_rng()
			.gen_range(0,len) as usize;
			index
	}
	fn quiz(quests:Vec<Question>,mut used_indices:Vec<String>){
		let mut questions:Vec<Question>=quests.clone();
		unsafe {NUM-=1};
		let mut terminate=false;
		 unsafe {if NUM <1 {
		 	terminate=true;
			}
		}
		let mut index:usize=0;
		loop{
			 index=get_index(questions.len());
			if used_indices.contains(&index.to_string()) {
				continue
			}
			else {
				break;
			}
		}
		let question=&questions[index];
		println!("\n {}",questions[index].question);
		used_indices.push(index.clone().to_string());
		for i in 0..question.options.len() as usize{
			println!(" {}: {}",i + 1,question.options[i]);
		}
		let input=take_input(questions.len());
		if question.options[input - 1]==question.answer{
			unsafe { SCORE+=1 }
		}
		if terminate{
			println!("\n================ Game Over =================\n" );
			unsafe{println!("           YOUR SCORE IS {} / 4", SCORE );};
			// println!("\n======== The Correct Answers Are ========\n");
			println!("\nDo you want to play again?(y/N)");
			let mut play=String::new();
			unsafe {
				io::stdin().read_line(&mut play).expect("Error");
				if play.trim() == "y".to_string() || play.trim() == "Y"{
					unsafe{NUM=4};
					used_indices=vec![];
					SCORE=0;
					quiz(quests,used_indices);
				}
				else {
					println!("\n=============== Quitting Game ==============");
				}
			}
		}
		else {
			quiz(questions,used_indices)
		}
	}
	fn take_input(len:usize)->usize{
		let mut input:usize=0;
		loop{
			let mut var=String::new();
			std::io::stdin().read_line(&mut var)
							.expect("Invalid Input");
			  input=match var.trim().parse(){
				Err(e) => {
					println!("Enter 1 2 3 or 4 as answer.");
					continue;
				},
				Ok(result) => result
			};
			if input >0 && input <5{
				break;
			}
			else {
				println!("Enter 1 2 3 or 4 as answer." );
				continue
			}
		}

		input
	}

quiz(questions,used_indices);
}