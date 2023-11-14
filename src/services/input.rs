use crate::{
    models::checkin::*,
    services::{
        cache::{insert_check_in, save_error},
        getid::get_booper_id,
    },
};
use chrono::{self, Local};
use std::io;
use threadpool::ThreadPool;

pub async fn wait_for_input() {
    let pool = ThreadPool::new(100);
    loop {
        println!("Input ID:");
        let mut id = String::new();

        io::stdin().read_line(&mut id).expect("failed to readline");

        pool.execute(move || save_input(id));
    }
}

fn save_input(mut id: String) {
    let id_length = 5;
    let booper_id: String = get_booper_id();

    id = id.trim().to_string();

    if id.len() == id_length {
        match id.parse::<i128>() {
            Ok(_) => {
                // correct id parameters
                let new_checkin: Checkin = Checkin {
                    id: booper_id,
                    student_id: id.trim().to_string(),
                    time_stamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
                };
                println!(
                    "id: {}, student_id: {}",
                    new_checkin.id, new_checkin.student_id
                );
                insert_check_in(&new_checkin);
            }
            Err(e) => {
                save_error(&AppError::new("Parsing Error".to_owned(), id));
                println!("Did not have a correct student id. Recieved: {}", e);
            }
        }
    } else if id.len() > id_length {
        let mod_id = &id[0..id_length];
        match mod_id.parse::<i128>() {
            Ok(_) => {
                // correct id parameters
                let new_checkin: Checkin = Checkin {
                    id: booper_id,
                    student_id: mod_id.trim().to_string(),
                    time_stamp: Local::now().format("%Y-%m-%d_%H:%M:%S.%3f").to_string(),
                };
                println!(
                    "id: {}, student_id: {}",
                    new_checkin.id, new_checkin.student_id
                );
                insert_check_in(&new_checkin);
            }
            Err(e) => {
                save_error(&AppError::new(
                    "Parsing Error & Length to long".to_owned(),
                    id,
                ));
                println!("Did not have a correct student id. Recieved: {}", e);
            }
        }
    } else {
        // Length to small
        save_error(&AppError::new("Length to small".to_owned(), id));
    }
}
