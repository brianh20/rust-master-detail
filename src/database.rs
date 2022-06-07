use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

use thiserror::Error;
use tui::{
    widgets::{
        ListState,
    },
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}

const DB_PATH: &str = "./data/db.json";

pub fn read_db() -> Result<Vec<Person>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Person> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn add_random_person_to_db() -> Result<Vec<Person>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<Person> = serde_json::from_str(&db_content)?;
    let catsdogs = match rng.gen_range(0, 1) {
        0 => "cats",
        _ => "dogs",
    };

    let random_person = Person {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: catsdogs.to_owned(),
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
    };

    parsed.push(random_person);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_person_at_index(person_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = person_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<Person> = serde_json::from_str(&db_content)?;
        if parsed.len() > 0 {
            parsed.remove(selected);
            fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;

            if selected == 0 {
                person_list_state.select(Some(selected));
            } else {
                person_list_state.select(Some(selected - 1));
            }
        }
    }
    Ok(())
}
