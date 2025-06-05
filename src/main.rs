mod utils;

use anyhow::Result;

// Testing
use utils::DeribitOption;

fn main() -> Result<()> {
    let option = "BTC-05JUN25-120000-C".parse::<DeribitOption>()?;
    println!("{:?}", option);

    Ok(())
}
