use diesel::Queryable;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::error::Error;
use crate::db::schema::pokemons::columns::name;
use crate::db::schema::pokemons::dsl::pokemons;

use crate::errors::app_error::AppError;

// ORM
#[derive(Queryable, Clone)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub types: String,
}

impl Pokemon {
    pub fn find_by_name(requested_name: String) -> Result<Pokemon, AppError> {
        let connection = establish_connection();
        return match pokemons.filter(name.eq(requested_name)).limit(1).load::<Pokemon>(&connection) {
            Ok(result) => match result.get(0) {
                Some(val) => Ok(Pokemon::clone(val)),
                None => Err(AppError::new_from_string("Pokemon with the name not found"))
            }
            Err(err) => Err(AppError::new(&err))
        };
    }
}

pub fn establish_connection() -> PgConnection {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
