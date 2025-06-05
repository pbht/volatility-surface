mod utils;

use anyhow::Result;

// Testing
use utils::RawDeribitOption;

fn main() -> Result<()> {
    let raw_option = RawDeribitOption {
        iv: 0.1,
        instrument_name: "BTC-05JUN25-120000-C".to_string(),
    };

    println!("{:?}", raw_option.into_full()?);

    Ok(())
}
