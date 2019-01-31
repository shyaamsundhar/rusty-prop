use chrono::{DateTime, Utc, TimeZone};
use chrono::serde::ts_seconds;

use crate::models::model_template::ModelTemplate;
use crate::models::kycdoc::Kycdoc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person{
    pub person_id:u32,
    pub name:String,
    #[serde(with = "ts_seconds")]
    pub dob:DateTime<Utc>,
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
            person_id: doc_id, 
            name:"dummy".to_string(),
            dob: Utc.ymd(1970,1,1).and_hms(0, 0, 0),
            docs: vec![Kycdoc::get(1)] 
        };
        some_var
    }
}