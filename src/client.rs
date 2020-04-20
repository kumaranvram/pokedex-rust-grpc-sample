pub mod pokedexpb;
use dotenv::dotenv;
use pokedexpb::poke_dex_client::PokeDexClient;
use pokedexpb::Query;
use std::env;
use crate::pokedexpb::{Pokemon, PokemonType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is mandatory");
    let mut client = PokeDexClient::connect(server_url).await?;

    let new_pokemon_request = tonic::Request::new(Pokemon{
        name: "Charizard".to_string(),
        pokemon_type: vec![PokemonType::Fire as i32],
    });
    let new_pokemon_response = client.make_pokedex_entry(new_pokemon_request).await?;
    println!("New Pokemon Entry Response = {:?}", new_pokemon_response);

    let get_pokemon_by_name_request = tonic::Request::new(Query {
        value: "Charizard".to_string(),
    });
    let get_pokemon_by_name_response = client.get_pokemon_by_name(get_pokemon_by_name_request).await?;
    println!("Get Pokemon By Name Response = {:?}", get_pokemon_by_name_response);

    let get_pokemons_by_type = tonic::Request::new(Query {
        value: "Fire".to_string()
    });
    let get_pokemons_by_type_response = client.get_pokemons_by_type(get_pokemons_by_type).await?;
    println!("Get Pokemons by Type Response = {:?}", get_pokemons_by_type_response);

    Ok(())
}
