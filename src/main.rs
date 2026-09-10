use reqwest::{self, Error};
use std::{fmt::format, io};

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
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city_name
    );
    let result = &url.replace("city_name", &city_name);
    // qual a diferenca de str para String?

    println!("DEBUG: {}", result);

    let resp = get_response(url.to_string()).await;

    match resp {
        Ok(r) => println!("ok, {r}"),
        Err(e) => println!("err"),
    }

    // println!("{resp}");
    //escrever o tipo do retorno do fetch
    // let response: HttpResponse = client.get(url).fetch().await?
}

//mudar para o fetch e remover o reqwest
async fn get_response(url: String) -> Result<String, Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
