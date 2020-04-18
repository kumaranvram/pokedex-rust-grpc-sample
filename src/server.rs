mod domain;
mod db;
mod errors;

use tonic::{transport::Server, Status, Response, Request};
use domain::poke_dex_server::PokeDex;
use domain::{PokemonResponse, PokemonsResponse, Query, Pokemon};
use domain::poke_dex_server::PokeDexServer;
use crate::domain::{PokedexEntryResponse, PokemonType};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = env::var("APP_PORT").expect("APP_PORT value must be set");
    let address = format!("0.0.0.0:{}", port).parse().unwrap();

    let context = PokeDexContext {};

    println!("Core Services listening on {}", address);

    Server::builder()
        .add_service(PokeDexServer::new(context.clone()))
        .serve(address)
        .await?;

    Ok(())
}

#[derive(Clone)]
pub struct PokeDexContext {}

#[tonic::async_trait]
impl PokeDex for PokeDexContext {
    async fn get_pokemon_by_name(
        &self,
        request: tonic::Request<Query>,
    ) -> Result<tonic::Response<PokemonResponse>, tonic::Status> {
        let requested_name = String::from(request.into_inner().value);
        match db::Pokemon::find_by_name(requested_name) {
            Ok(pokemon) => Result::Ok(tonic::Response::new(PokemonResponse {
                id: pokemon.id,
                name: pokemon.name,
                pokemon_type: to_pokemon_types(pokemon.types),
            })),
            Err(err) => Result::Err(Status::not_found(err.to_string()))
        }
    }

    async fn get_pokemons_by_type(&self, request: Request<Query>) -> Result<Response<PokemonsResponse>, Status> {
        let requested_type = String::from(request.into_inner().value);
        match db::Pokemon::find_by_type(requested_type) {
            Ok(pokemons) => {
                Result::Ok(tonic::Response::new(PokemonsResponse {
                    pokemons: pokemons.into_iter().map(|p| {
                        return PokemonResponse {
                            id: p.id,
                            name: p.name,
                            pokemon_type: to_pokemon_types(p.types),
                        };
                    }).collect()
                }))
            }
            Err(err) => Result::Err(Status::not_found(err.to_string()))
        }
    }

    async fn make_pokedex_entry(&self, request: Request<Pokemon>) -> Result<Response<PokedexEntryResponse>, Status> {
        let pokemon = request.into_inner();

        let pokemon_types: Vec<String> = pokemon.pokemon_type.iter()
            .map(|t| get_pokemon_type_string(*t)).collect();
        let types = pokemon_types.join("|");
        match db::Pokemon::save(pokemon.name, types) {
            Ok(id) => Result::Ok(tonic::Response::new(PokedexEntryResponse {
                id
            })),
            Err(err) => Result::Err(Status::not_found(err.to_string()))
        }
    }
}

fn to_pokemon_types(types: String) -> Vec<i32> {
    types.split("|").map(|s| find_pokemon_type(s.to_string()) as i32).collect()
}

fn find_pokemon_type(pokemon_type: String) -> PokemonType {
    return match String::from(pokemon_type.to_uppercase()).as_str() {
        "FIRE" => PokemonType::Fire,
        "GROUND" => PokemonType::Ground,
        "WATER" => PokemonType::Water,
        "GRASS" => PokemonType::Grass,
        "PSYCHIC" => PokemonType::Psychic,
        "GHOST" => PokemonType::Ghost,
        "ICE" => PokemonType::Ice,
        "STEEL" => PokemonType::Steel,
        "POISON" => PokemonType::Poison,
        "FLYING" => PokemonType::Flying,
        "FIGHTING" => PokemonType::Fighting,
        "ELECTRIC" => PokemonType::Electric,
        _ => PokemonType::Normal,
    };
}

fn get_pokemon_type_string(pokemon_type: i32) -> String {
    return match pokemon_type {
        1 => String::from("FIRE"),
        2 => String::from("GROUND"),
        3 => String::from("WATER"),
        4 => String::from("GRASS"),
        5 => String::from("PSYCHIC"),
        6 => String::from("GHOST"),
        7 => String::from("ICE"),
        8 => String::from("STEEL"),
        9 => String::from("POISON"),
        10 => String::from("FLYING"),
        11 => String::from("FIGHTING"),
        12 => String::from("ELECTRIC"),
        _ => String::from("NORMAL"),
    };
}