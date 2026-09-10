use core::error;
use reqwest::{self, Error};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiErrors {
    #[error("deu erro otario {0}")]
    Rede(#[from] reqwest::Error),
    #[error("erro no jason {0}")]
    Parse(#[from] serde_json::Error),
}
#[derive(serde::Deserialize, Debug)]
struct GeoResult {
    id: i32,
    name: String,
    latitude: f64,
    longitude: f64,
    country: String,
    admin1: String,
}

#[derive(serde::Deserialize)]
struct ApiResponse {
    results: Option<Vec<GeoResult>>,
    generationtime_ms: f64,
    reason: Option<String>,
    error: Option<bool>,
}

#[tokio::main]
async fn main() {
    println!("Bem vindo a wetRust weather forecast");
    println!("Digite o nome da cidade que voce quer pesquisar:");

    let mut city_name = String::new();
    io::stdin()
        .read_line(&mut city_name)
        .expect("Faield to read");
    let city_name = city_name.trim();
    println!("A consulta será feita para a cidade de {city_name}");

    // let url = "https://geocoding-api.open-meteo.com/v1/search?name={city_name}";

    // USAR FORMAT! para montar a url
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        city_name
    );
    // qual a diferenca de str para String?

    // println!("DEBUG: {}", result);

    let resp = get_response(url.to_string()).await;

    //estudar mais match
    // - entender se é padrão para fazer "try/catch"
    // - tratamento de dados
    let dados = match resp {
        Ok(r) => r,
        Err(e) => {
            println!("ERRO:{e}");
            return;
        }
    };
    //criar uma lista de seleção para as cidades encontradas na busca, e fazer outra busca na api para definir o clima no local
    println!("Cidade encontrada, resultado {:#?}", &dados.results);
    //escrever o tipo do retorno do fetch
    // let response: HttpResponse = client.get(url).fetch().await?
}

//mudar para o fetch e remover o reqwest
async fn get_response(url: String) -> Result<ApiResponse, ApiErrors> {
    let body = reqwest::get(url).await?.text().await?;
    let data: ApiResponse = serde_json::from_str(&body)?;

    Ok(data)
}
