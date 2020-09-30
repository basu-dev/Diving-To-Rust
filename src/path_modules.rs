use uuid::Uuid;
pub fn run(){
    mod School{
        mod Education{
            use super::super::Uuid;
            enum Grade{
                One,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten
            }
            struct Student{
                id:uuid::Uuid,
                name:String,
                grade:Grade
            }
            impl Student{
                fn register(name:String,grade:Grade)->Student{
                    Student{
                        id:Uuid::new_v4(),
                        name,
                        grade
                    }
                }
                
            }
        }
        mod Finance{
            enum Position{
                Principal(u32),Teacher(u32),Accountant(u32)
            }
            struct Employee{
                name:String,
                position:Position,
            }

        }
    }
}