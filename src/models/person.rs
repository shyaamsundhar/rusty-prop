use chrono::prelude::Date;
use chrono::prelude::Local;
use crate::models::model_template::ModelTemplate;
use crate::models::kycdoc::Kycdoc;

#[derive(Debug)]
pub struct Person{
    pub id:u32,
    pub name:String,
    pub dob:Date<Local>,
    pub docs:Vec<Kycdoc>
}

impl ModelTemplate for Person{
    fn save(&self) -> u32{
        1
    }
    fn update(&self) -> bool{
        true
    }
    fn get(doc_id:u32) -> Self{
        let some_var = Person {
            id: doc_id, 
            name:"dummy".to_string(),
            dob: Local::today(), 
            docs: vec![Kycdoc {id: doc_id, name:"dummy".to_string(), value: Local::now()}] 
        };
        some_var
    }
}