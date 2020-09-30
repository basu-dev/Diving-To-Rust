use uuid::Uuid;
pub mod Education{
    use super::Uuid;
    #[derive(Debug)]
   pub enum Grade{
        One,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten
    }
    #[derive(Debug)]
    pub struct Student{
       pub id:uuid::Uuid,
       pub name:String,
       pub grade:Grade
    }
    impl Student{
        pub fn new(name:String,grade:Grade)->Student{
            Student{
                 id:Uuid::new_v4(),
                name,
                grade
            }
        }
        pub fn get_by_id(students:&Vec<&Student>,id:Uuid)->Option<Student>{
            for (i,&student) in students.iter().enumerate(){
                if student.id==id {
                    return Some(student)
                }
            }
            None
        }
    }
}