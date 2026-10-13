use core::error;
use reqwest::{self, Error};
use serde_json::error::Category::Data;
use std::io;
use thiserror::Error;
use chrono::prelude::*;
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

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    results: Option<Vec<GeoResult>>,
    generationtime_ms: f64,
    reason: Option<String>,
    error: Option<bool>,
}
#[derive(serde::Deserialize, Debug)]
struct WeatherResult {
    hourly: Hourly,
}


#[derive(serde::Deserialize, Debug)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    precipitation_probability: Vec<f64>,
    rain: Vec<f64>,
    snowfall: Vec<f64>,
    apparent_temperature: Vec<f64>,
}

#[derive(Debug)]
struct CurrentWeather {
    time: String,
    temperature_2m: f64 ,
    precipitation_probability: f64,
    rain: f64,
    snowfall: f64, 
    apparent_temperature: f64,
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

    // qual a diferenca de str para String?

    // !("DEBUG: {}", result);

    let resp = get_city(city_name.to_string()).await;

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

    //dbg!(&dados);
    if dados.results.is_none() {
        println!("Nenhuma cidade encontrada.");
        return;
        // volta ao fluxo de escolher cidade
    }

    //
    let cidades = dados.results.unwrap();

    println!("Digite o número da cidade que voce quer pesquisar");

    for (i, cidade) in cidades.iter().enumerate() {
        println!(
            "{} para, {}, {}, {}",
            i, cidade.name, cidade.admin1, cidade.country
        );
    }

    // pegar input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("digite o número da opcao");
    let input: usize = input.trim().parse().unwrap();

    let cidade_escolhida = cidades.get(input).unwrap();
    // println!("Voce escolheu a cidade {:#?}", cidade_escolhida);
    dbg!(cidade_escolhida);
    // return;
    // dbg!(cidades);
    let dados = get_weather(cidade_escolhida).await;
    dbg!(dados);
    //criar uma lista de seleção para as cidades encontradas na busca, e fazer outra busca na api para definir o clima no local
    // let var_name = println!("Cidade encontrada, resultado {:#?}", &dados.results);
}

// refactor de nome de função
async fn get_city(city_name: String) -> Result<ApiResponse, ApiErrors> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        city_name
    );

    let body = reqwest::get(url).await?.text().await?;

    let data: ApiResponse = serde_json::from_str(&body)?;

    Ok(data)
}


async fn get_weather(cidade_escolhida: &GeoResult) -> Result<WeatherResult, ApiErrors> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,precipitation_probability,rain,snowfall,apparent_temperature,visibility&forecast_days=1",
        cidade_escolhida.latitude, cidade_escolhida.longitude
    );
    let body = reqwest::get(url).await?.text().await?;

    let data: WeatherResult = serde_json::from_str(&body)?;

    Ok(data)
}

async fn get_current_weather(data: &WeatherResult) -> Option<CurrentWeather> { 
    // let current_time = chrono::Local::now().format("%Y-%m-%dT%H:00").to_string();
    // let index = data.hourly.time.iter().position(|t| t == &current_time)?;
    let current_hour2 = chrono::Local::now().format("%Y-%m-%dT%H:00").to_string();
    let index = data.hourly.time.iter().position(|t| t == &current_hour2)?;
    Some(CurrentWeather {
        time: current_hour2,
        temperature_2m: data.hourly.temperature_2m[index],
        precipitation_probability: data.hourly.precipitation_probability[index],
        rain: data.hourly.rain[index],
        snowfall: data.hourly.snowfall[index],
        apparent_temperature: data.hourly.apparent_temperature[index],
    })
}
