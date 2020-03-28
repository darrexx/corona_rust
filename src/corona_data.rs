
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct Location{
    pub country: String,
    pub countryInfo: CountryInfo,
    pub cases: i32,
    pub todayCases: i32,
    pub deaths: i32,
    pub todayDeaths: i32,
    pub recovered: i32,
    pub active: i32,
    pub critical: i32,
    pub casesPerOneMillion: f64,
    pub deathsPerOneMillion: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CountryInfo{
    pub iso2: Option<String>,
}