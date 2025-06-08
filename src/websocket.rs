use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use crate::types::{DeribitWebSocketMessage, RawDeribitOption};

pub async fn listen_for_deribit_data(tx: mpsc::Sender<Vec<RawDeribitOption>>) -> Result<()> {
    let subscribe_message = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "public/subscribe",
        "params": {
            "channels": ["markprice.options.btc_usd"]
        }
    });

    let (mut ws_stream, _) =
        tokio_tungstenite::connect_async("wss://test.deribit.com/ws/api/v2").await?;
    println!("Connected to deribit");

    ws_stream
        .send(Message::Text(subscribe_message.to_string().into()))
        .await?;
    println!("Listening for deribit data");

    let (_, mut read) = ws_stream.split();
    while let Some(Ok(message)) = read.next().await {
        let message = message.to_text()?;
        let message: DeribitWebSocketMessage = serde_json::from_str(message)?;

        if let Some(params) = message.params {
            tx.send(params.data)?;
        }
    }

    Ok(())
}
