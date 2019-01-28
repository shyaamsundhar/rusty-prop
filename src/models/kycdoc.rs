//mod model_template;

use chrono::prelude::DateTime;
use chrono::prelude::Local;
use crate::models::model_template::ModelTemplate;

#[derive(Debug)]
pub struct Kycdoc{
    pub id:u32,
    pub name:String,
    pub value:DateTime<Local>
}

impl ModelTemplate for Kycdoc{
    fn save(&self) -> u32{
        1
    }
    fn update(&self) -> bool{
        true
    }
    fn get(doc_id:u32) -> Self{
        let some_var = Kycdoc {id: doc_id, name:"dummy".to_string(), value: Local::now()};
        some_var
    }
}