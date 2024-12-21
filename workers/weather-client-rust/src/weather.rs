use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub day: String,
    pub temperature: String,
    pub wind: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub temperature: String,
    pub wind: String,
    pub description: String,
    pub forecast: Vec<Forecast>,
}

#[derive(Debug)]
pub enum WeatherError {
    FetchError(worker::Error),
    JsonParseError(String),
    EmptyResponse,
}

impl From<worker::Error> for WeatherError {
    fn from(err: worker::Error) -> Self {
        WeatherError::FetchError(err)
    }
}

impl From<WeatherError> for worker::Error {
    fn from(err: WeatherError) -> Self {
        match err {
            WeatherError::FetchError(e) => e,
            WeatherError::JsonParseError(msg) => worker::Error::from(msg),
            WeatherError::EmptyResponse => worker::Error::from("Empty response received"),
        }
    }
}

pub async fn get_weather(city: &str) -> Result<Weather> {
    let url = format!("https://goweather.herokuapp.com/weather/{}", city);
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Accept", "application/json")?;

    let request = Request::new_with_init(
        &url,
        RequestInit::new()
            .with_method(Method::Get)
            .with_headers(headers),
    )?;

    let mut resp = Fetch::Request(request).send().await?;
    console_debug!("Response status: {}", resp.status_code());
    console_debug!("Response headers: {:#?}", resp.headers());

    let text = resp.text().await?;
    console_debug!("Response body: {}", text);
    
    if text.trim().is_empty() {
        return Err(WeatherError::EmptyResponse.into());
    }
    
    serde_json::from_str(&text)
        .map_err(|e| WeatherError::JsonParseError(e.to_string()).into())
}
