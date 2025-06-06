use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate};
use std::str::FromStr;

use crate::types::{
    DeribitDataPoint, DeribitOptionStringObject, FullDeribitOption, OptionSide, RawDeribitOption,
};

impl RawDeribitOption {
    pub fn into_full(self) -> Result<FullDeribitOption> {
        let option_string = self.instrument_name.parse::<DeribitOptionStringObject>()?;

        Ok(FullDeribitOption {
            underlying: option_string.underlying,
            expiry: option_string.expiry,
            strike: option_string.strike,
            side: option_string.side,
            iv: self.iv,
        })
    }
}

impl FromStr for DeribitOptionStringObject {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();

        match parts.as_slice() {
            [raw_underlying, raw_expiry, raw_strike, raw_side] => {
                let underlying = raw_underlying.to_string();
                let expiry = NaiveDate::parse_from_str(raw_expiry, "%e%b%y")?;

                let strike = raw_strike.parse::<f64>()?;

                let side = match *raw_side {
                    "C" => OptionSide::Call,
                    "P" => OptionSide::Put,
                    _ => {
                        return Err(anyhow!("Invalid option side"));
                    }
                };

                Ok(DeribitOptionStringObject {
                    underlying,
                    expiry,
                    strike,
                    side,
                })
            }
            _ => Err(anyhow!("Invalid option format: {}", s)),
        }
    }
}

impl FullDeribitOption {
    pub fn into_data_point(self) -> DeribitDataPoint {
        let today = Local::now().date_naive();
        let converted_date = (self.expiry - today).num_days() as f64;

        DeribitDataPoint {
            x: self.strike,
            y: converted_date,
            z: self.iv as f32,
        }
    }
}
