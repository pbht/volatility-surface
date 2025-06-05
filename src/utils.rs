use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use std::str::FromStr;

// Not explicitly used just yet, but may be useful in the future.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionSide {
    Call,
    Put,
}

// This is the raw data that we receive from the Deribit websocket API over the course of (potentially) multiple vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct RawDeribitOption {
    pub iv: f64,
    pub instrument_name: String,
}

// This is an intermediate 'translation' struct that we use to get from the raw data to the full data.
#[derive(Debug, Clone, PartialEq)]
struct DeribitOptionStringObject {
    underlying: String,
    expiry: NaiveDate,
    strike: f64,
    side: OptionSide,
}

// This struct encodes all the data (+ some extra) that we need to plot each point in the surface.
#[derive(Debug, Clone, PartialEq)]
pub struct FullDeribitOption {
    underlying: String,
    expiry: NaiveDate,
    strike: f64,
    side: OptionSide,
    iv: f64,
}

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
