use rand::Rng;
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
	fn quiz(questions:Vec<Question>){
		let index=rand::thread_rng().gen_range(0,questions.len()) as usize;
		let question=questions[index];
		println!("{:?}",questions[index].question);

		println!("{:?}",questions[index].options );
	}
	quiz(questions);
}