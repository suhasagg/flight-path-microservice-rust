use clap::Parser;

/// Flight program to calculate the total path of the airport flights.
#[derive(Parser, Debug)]
#[clap(about, version, author, long_about = None)]
pub struct Args {
    /// HTTP address to listen on
    #[clap(short, long, default_value = "0.0.0.0:2333", env = "FLIGHT_ADDR")]
    pub addr: String,
}

pub fn parse() -> Args {
    let args = Args::parse();
    // initialize tracing
    tracing_subscriber::fmt::init();
    args
}
