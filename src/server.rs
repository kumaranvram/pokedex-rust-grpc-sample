mod domain;
use tonic::{transport::Server, Status, Response, Request};
use domain::poke_dex_server::{PokeDex};
use domain::{PokemonResponse, PokemonsResponse, Query, Pokemon, PokemonType};
use domain::poke_dex_server::PokeDexServer;
use crate::domain::PokedexEntryResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = PokeDexContext{};

    let addr = "0.0.0.0:5000".parse().unwrap();

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
        Result::Ok(tonic::Response::new(PokemonResponse{
            id: 42,
            name: String::from(request.into_inner().value),
            pokemon_type: vec![PokemonType::Flying.into(), PokemonType::Poison.into()]
        }))
    }

    async fn get_pokemons_by_type(&self, _: Request<Query>) -> Result<Response<PokemonsResponse>, Status> {
        unimplemented!()
    }

    async fn make_pokedex_entry(&self, _: Request<Pokemon>) -> Result<Response<PokedexEntryResponse>, Status> {
        unimplemented!()
    }
}

// fn main() {
//     println!("Gotta catch'em all!!");
// }
