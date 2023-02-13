use chrono::{self, Local};

pub struct Checkin {
    pub mac_address: String,
    pub student_id: String,
    pub time_stamp: String
}

#[derive(Clone, Debug)]
pub struct AppError {
    pub id: Option<u64>,
    pub etype: String,
    pub input: String,
    pub time: String,
    pub received: u8
}

impl AppError {
    pub fn new(etype: String, input: String) -> Self{
        Self {
            id: None,
            etype,
            input,
            time: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            received: 0,
        }
    }
}