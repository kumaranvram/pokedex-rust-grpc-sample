mod domain;
mod db;
mod errors;

use tonic::{transport::Server, Status, Response, Request, Code};
use domain::poke_dex_server::{PokeDex};
use domain::{PokemonResponse, PokemonsResponse, Query, Pokemon};
use domain::poke_dex_server::PokeDexServer;
use crate::domain::PokedexEntryResponse;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = env::var("APP_PORT").expect("APP_PORT value must be set");
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    let context = PokeDexContext{};

    println!("Core Services listening on {}", addr);

    Server::builder()
        .add_service(PokeDexServer::new(context.clone()))
        .serve(addr)
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
            Ok(pokemon) => Result::Ok(tonic::Response::new(PokemonResponse{
                id: pokemon.id,
                name: pokemon.name,
                pokemon_type: to_pokemon_types(pokemon.types)
            })),
            Err(err) => Result::Err(Status::not_found(err.description()))
        }
    }

    async fn get_pokemons_by_type(&self, _: Request<Query>) -> Result<Response<PokemonsResponse>, Status> {
        unimplemented!()
    }

    async fn make_pokedex_entry(&self, _: Request<Pokemon>) -> Result<Response<PokedexEntryResponse>, Status> {
        unimplemented!()
    }
}

fn to_pokemon_types(types: String) -> Vec<i32> {
    types.split(",").map(|s| s.parse().unwrap()).collect()
}
