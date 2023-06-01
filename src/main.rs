use reqwest::Error;
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, Write};
use tokio::runtime::Runtime;

fn get_opposite_pairs() -> HashMap<&'static str, &'static str> {
    let mut opposite_pairs: HashMap<&'static str, &'static str> = HashMap::new();
    opposite_pairs.insert("fantasy", "non-fiction");
    opposite_pairs.insert("romance", "thriller");
    opposite_pairs.insert("young-adult", "classic");
    // Adicione mais pares conforme necessário
    opposite_pairs
}

async fn get_book_by_genre(genre: &str) -> Result<Value, Error> {
    let url = format!("https://openlibrary.org/subjects/{}.json", genre);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    let works = resp["works"].as_array().unwrap();
    let book = rand::thread_rng().gen_range(0..works.len());
    Ok(works[book].clone())
}

async fn recommend_book(user_genre: &str) -> Result<(), Error> {
    let opposite_pairs = get_opposite_pairs();
    let opposite_genre = match opposite_pairs.get(user_genre) {
        Some(genre) => genre,
        None => {
            println!("Desculpe, não tenho um par de oposição para o gênero {}.", user_genre);
            return Ok(());
        }
    };
    let book = get_book_by_genre(opposite_genre).await?;
    println!(
        "Como você gosta de {}, talvez não goste de {} ({}).",
        user_genre, book["title"], opposite_genre
    );
    Ok(())
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let mut input = String::new();
    loop {
        print!("Qual gênero de livro você está lendo agora? (Digite 'sair' para sair) ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let genre = input.trim();
        if genre == "sair" {
            break;
        }
        runtime.block_on(recommend_book(genre)).unwrap();
        input.clear();
    }
}
