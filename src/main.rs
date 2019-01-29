extern crate chrono;
mod models;

use models::kycdoc;
use chrono::prelude::Local;

fn main() {
    let check = kycdoc::Kycdoc { kyc_id:1, name:"dummy".to_string(), value: Local::now() };
    println!("{:?}", check);
}