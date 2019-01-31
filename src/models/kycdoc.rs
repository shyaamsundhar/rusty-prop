use crate::models::model_template::ModelTemplate;

//use serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kycdoc{
    pub kyc_id:u32,
    pub name:String,
    pub value:String
}

impl ModelTemplate for Kycdoc{
    fn save(&self) -> u32{
        1
    }
    fn update(&self) -> bool{
        true
    }
    fn get(doc_id:u32) -> Self{
        let some_var = Kycdoc {kyc_id: doc_id, name:"dummy".to_string(), value: "valuesss".to_string()};
        some_var
    }
}