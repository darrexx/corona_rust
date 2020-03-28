
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Location{
    id: usize,
    pub country: String,
    pub country_code: String,
    pub country_population: Option<u32>,
    pub province: String,
    pub latest: LatestCoronaData,
}

#[derive(Deserialize,Debug, Clone)]
pub struct LatestCoronaData{
    pub confirmed : isize, //possible -1!
    pub deaths: isize,
    pub recovered: isize,
}

#[derive(Deserialize)]
pub struct CoronaResponse{
    pub locations: Vec<Location>
}