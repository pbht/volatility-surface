use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionSide {
    Call,
    Put,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeribitOption {
    underlying: String,
    expiry: NaiveDate,
    strike: f64,
    side: OptionSide,
}

impl DeribitOption {
    fn new(underlying: String, expiry: NaiveDate, strike: f64, side: OptionSide) -> Self {
        Self {
            underlying,
            expiry,
            strike,
            side,
        }
    }
}

impl FromStr for DeribitOption {
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
                        return Err(anyhow!("Invalid option side: missing side"));
                    }
                };

                Ok(DeribitOption::new(underlying, expiry, strike, side))
            }
            _ => Err(anyhow!("Invalid option format: {}", s)),
        }
    }
}
