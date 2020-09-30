
   pub mod Finance{
#[derive(Debug)]
       pub enum Position{
            Principal(u32),Teacher(u32),Accountant(u32)
        }
        #[derive(Debug)]

       pub struct Employee{
            name:String,
            position:Position,
        }
        impl Employee{
            pub fn new(name:String,position:Position)->Employee{
                Employee{
                    name,position
                }
            }
        }
    }
