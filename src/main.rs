mod corona_data;

use reqwest;
use dipstick::{Prefixed, Graphite, Input, InputScope};
use corona_data::corona_data::{CoronaResponse, Location};

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
    write_location_data(locations, metrics).unwrap();
    Ok(())
}

fn write_location_data(locations: Vec<Location>, metrics: Graphite) -> Result<(), String> {
    for location in locations.iter(){
        if location.country_code == "DE" || location.country_code == "US" || location.country_code == "DK" || location.country_code == "CR" {
            println!("{}", location.country_code);
            println!("{:?}", location);
        }
        let confirmed = if location.latest.confirmed < 0 {
            0
        }
        else{
            location.latest.confirmed
        };
        let deaths = if location.latest.deaths < 0 {
            0
        }
        else{
            location.latest.deaths
        };
        let recovered = if location.latest.recovered < 0 {
            0
        }
        else{
            location.latest.recovered
        };
        
        let location_metrics = metrics.add_name(format!("{}", location.country_code)).metrics();

        location_metrics.counter("confirmed").count(confirmed as usize);
        std::thread::sleep(std::time::Duration::from_millis(10));
        location_metrics.counter("deaths").count(deaths as usize);
        std::thread::sleep(std::time::Duration::from_millis(10));
        location_metrics.counter("recovered").count(recovered as usize);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
