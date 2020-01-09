use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "riot", about = "Riot Games (League of Legends) API CLI")]
pub enum Riot {
    #[structopt(name = "status", about="Fetch service status information")]
    Status {
        #[structopt(short = "r")]
        region: Option<String>,
    }
}