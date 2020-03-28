mod corona_data;

use reqwest;
use dipstick::{Prefixed, Graphite, Input, InputScope};
use corona_data::Location;

#[derive(Debug)]
struct CoronaMetricData{
    country_code: String,
    confirmed: usize,
    deaths: usize,
    recovered: usize,
    active: usize,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let metrics = Graphite::send_to("localhost:2003")
        .expect("Connected")
        .named("corona");

    let body = reqwest::get("https://corona.lmao.ninja/countries")
        .await?
        .text()
        .await?;

    let locations: Vec<Location> = serde_json::from_str(&body).unwrap();
    
    let metric_data = preapre_data(&locations);



    write_location_data(metric_data, metrics).unwrap();
    Ok(())
}

fn preapre_data(locations: &Vec<Location>) -> Vec<CoronaMetricData>{

    locations.iter().map(|x| CoronaMetricData{
        country_code:  match x.clone().countryInfo.iso2 {
            Some(code) => code,
            None => "NONE".to_string()
        },
        confirmed: x.cases.max(0) as usize,
        recovered: x.recovered.max(0) as usize,
        deaths: x.deaths.max(0) as usize,
        active: x.active.max(0) as usize,
    }).collect()
}


fn write_location_data(data: Vec<CoronaMetricData>, graphite: Graphite) -> Result<(), String> {
    let metrics = graphite.metrics();
    for metric in data.iter(){

        let location_metrics = metrics.add_name(format!("{}", metric.country_code));

        location_metrics.counter("confirmed").count(metric.confirmed );
        std::thread::sleep(std::time::Duration::from_millis(500));

        location_metrics.counter("deaths").count(metric.deaths);
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        location_metrics.counter("recovered").count(metric.recovered);
        std::thread::sleep(std::time::Duration::from_millis(500));

        location_metrics.counter("active").count(metric.active);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
