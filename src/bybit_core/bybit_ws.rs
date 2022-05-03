use anyhow::{Error as AnyHowError, Result as AnyHowResult};
use bybit::ws;
use futures_util::StreamExt;

pub async fn bybit_websocket(
	bybit_pub_key: String,
	bybit_priv_key: String,
) -> AnyHowResult<(), AnyHowError> {
	let mut api = ws::Client::new(ws::MAINNET_BYBIT, &bybit_pub_key, &bybit_priv_key);
	api.connect().await?;

	api.subscribe(&[ws::Channel::OrderBook25("BTCUSD".to_string())])
		.await?;

	for _ in 0..5 {
		if let Some(Ok(data)) = api.next().await {
			println!("{:?}", data)
		}
	}

	api.disconnect().await?;

	//tbf
	Ok(())
}
