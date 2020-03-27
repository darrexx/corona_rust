
pub mod corona_data{
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Location{
        id: usize,
        pub country: String,
        pub country_code: String,
        pub country_population: Option<u32>,
        pub province: String,
        pub latest: LatestCoronaData,
    }
    
    #[derive(Deserialize,Debug)]
    pub struct LatestCoronaData{
        pub confirmed : i32, //possible -1!
        pub deaths: i32,
        pub recovered: i32,
    }
    
    #[derive(Deserialize)]
    pub struct CoronaResponse{
        pub locations: Vec<Location>
    }

}