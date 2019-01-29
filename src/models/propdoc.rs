use chrono::prelude::Date;
use chrono::prelude::Local;
use crate::models::model_template::ModelTemplate;
use crate::models::person::Person;

//Enum definitions
pub enum Measurements{
    HectAcre,
    Acre,
    Cent
}
pub enum LandTypes{
    Wet,
    Dry
}

//Enum definitions ends here

pub struct Borders{
    pub north:String,
    pub east:String,
    pub west:String,
    pub south:String
}

pub struct RegistrarBook{
    pub volume:String,
    pub year:String,
    pub page_from:u32,
    pub page_to:u32
}

pub struct PropDoc{
    pub prop_id:u32,
    pub friendly_name:String,
    pub is_multiple_holders: bool,
    pub holders:Vec<Person>,
    pub date_of_registration:Date<Local>,
    pub village:String,
    pub town:String,
    pub taluk:String,
    pub rs_no:String,
    pub sub_no:String,
    pub measurement_type:Measurements,
    pub measurement_value:f32,
    pub border_desc:Borders,
    pub land_type:LandTypes,
    pub natham:String,
    pub registrar_book:RegistrarBook,
    pub encumbrance_certificate:String,
    pub fieldmap:String
}

impl ModelTemplate for PropDoc{
    fn save(&self) -> u32{
        1
    }
    fn update(&self) -> bool{
        true
    }
    fn get(doc_id:u32) -> Self{
        let some_var = PropDoc {
            prop_id: doc_id, 
            friendly_name: "some_friendly_name".to_string(),
            is_multiple_holders: false,
            holders: vec![Person::get(1)],
            date_of_registration:Local::today(),
            village:"my_village".to_string(),
            town:"my_town".to_string(),
            taluk:"my_taluk".to_string(),
            rs_no:"some_rs_no".to_string(),
            sub_no:"some_sub_no".to_string(),
            measurement_type: Measurements::HectAcre,
            measurement_value:125.12,
            border_desc: Borders { north: "".to_string(), east:"".to_string(), west:"".to_string(), south:"".to_string() },
            land_type: LandTypes::Wet,
            natham:"abcd".to_string(),
            registrar_book: RegistrarBook { volume:"1".to_string(), year: "2019".to_string(), page_from: 1, page_to: 10 },
            encumbrance_certificate: "ec cert".to_string(),
            fieldmap:"fldmap".to_string()
        };
        some_var
    }
}