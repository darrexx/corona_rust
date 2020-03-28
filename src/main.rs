mod corona_data;

use reqwest;
use dipstick::{Prefixed, Graphite, Input, InputScope};
use corona_data::{CoronaResponse, Location};
use itertools::Itertools;

#[derive(Debug)]
struct CoronaMetricData{
    country_code: String,
    confirmed: usize,
    deaths: usize,
    recovered: usize,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let metrics = Graphite::send_to("localhost:2003")
        .expect("Connected")
        .named("corona");

    let body = reqwest::get("https://coronavirus-tracker-api.herokuapp.com/v2/locations")
        .await?
        .text()
        .await?;

    let corona: CoronaResponse = serde_json::from_str(&body).unwrap();
    
    let locations = corona.locations;
    let metric_data = preapre_data(&locations);



    write_location_data(metric_data, metrics).unwrap();
    Ok(())
}

fn preapre_data(locations: &Vec<Location>) -> Vec<CoronaMetricData>{

    locations.into_iter()
        .group_by(|x| x.country_code.clone()).into_iter()
        .map(|(x, y) | {
            let location = y.cloned().fold1(
                |mut acc, metric| {
                    acc.latest.confirmed += metric.latest.confirmed.max(0);
                    acc.latest.recovered += metric.latest.confirmed.max(0);
                    acc.latest.deaths += metric.latest.confirmed.max(0);
                    acc
                }
            ).unwrap();
            return CoronaMetricData{country_code : x,
                confirmed : location.latest.confirmed as usize,
                deaths : location.latest.deaths as usize,
                recovered : location.latest.recovered as usize,
            }}).collect()
}


fn write_location_data(data: Vec<CoronaMetricData>, graphite: Graphite) -> Result<(), String> {
    let metrics = graphite.metrics();
    for metric in data.iter(){

        let location_metrics = metrics.add_name(format!("{}", metric.country_code));

        location_metrics.counter("confirmed").count(metric.confirmed );
        std::thread::sleep(std::time::Duration::from_millis(1000));

        location_metrics.counter("deaths").count(metric.deaths);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        location_metrics.counter("recovered").count(metric.recovered);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
