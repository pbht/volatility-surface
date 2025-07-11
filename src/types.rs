use chrono::NaiveDate;
use clap::{ArgAction, Parser};
use serde::Deserialize;

#[derive(Parser)]
pub struct Args {
    #[arg(long, action = ArgAction::Set, default_value_t = true)]
    pub puts: bool,
    #[arg(long, action = ArgAction::Set, default_value_t = true)]
    pub calls: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionSide {
    Call,
    Put,
}

// This is the raw data that we receive from the Deribit websocket API over the course of (potentially) multiple vectors.
// We will coerce the websocket response into this struct.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeribitWebSocketMessage {
    pub params: Option<RawDeribitOptionData>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RawDeribitOptionData {
    pub data: Vec<RawDeribitOption>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RawDeribitOption {
    pub iv: f64,
    pub instrument_name: String,
}

// This is an intermediate 'translation' struct that we use to get from the raw data to the full data.
#[derive(Debug, Clone, PartialEq)]
pub struct DeribitOptionStringObject {
    pub underlying: String,
    pub expiry: NaiveDate,
    pub strike: f64,
    pub side: OptionSide,
}

// This struct encodes all the data (+ some extra) that we need to plot each point in the surface.
#[derive(Debug, Clone, PartialEq)]
pub struct FullDeribitOption {
    pub underlying: String,
    pub expiry: NaiveDate,
    pub strike: f64,
    pub side: OptionSide,
    pub iv: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeribitDataPoint {
    pub x: f64,
    pub y: f64,
    pub z: f32,
}
