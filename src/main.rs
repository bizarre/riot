#[macro_use]
extern crate serde_derive;

use riven::RiotApi;
use riven::consts::Region;

mod cli;
mod settings;

use std::collections::HashMap;
use std::str::FromStr;
use cli::Riot;

use colored::*;

use structopt::StructOpt;
use settings::Settings;

fn main() {
    let settings = Settings::new().unwrap();
    let api_key = settings.api_key;

    if api_key.is_none() {
        println!("{:}", "API key not defined.".red());
        return
    }

    match Riot::from_args() {
        Riot::Status { region } => {
            let region = region.unwrap_or(settings.region.unwrap());
            let region_enum = region.parse::<Region>();

            if region_enum.is_err() {
                println!("{:}", "Invalid region".red());
                return;
            }
            
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let riot_api = RiotApi::with_key(api_key.unwrap());

                let shard = riot_api.lol_status_v3()
                    .get_shard_data(region_enum.unwrap()).await
                    .expect("Failed to get league status.");

                println!("{:} {:}", shard.name.red(), "Status".red());
                
                for service in shard.services {
                    match service.status.as_ref() {
                        "online" => println!("{:}: {:}", service.name.white(), service.status.green()),
                        _ => println!("{:}: {:}", service.name.white(), service.status.red())
                    }
                
                }
            });
        }
    }
}
