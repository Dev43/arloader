use arloader::transaction::{stringify, Base64};
use reqwest;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Offset {
    #[serde(with = "stringify")]
    size: usize,
    #[serde(with = "stringify")]
    offset: usize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct RawChunk {
    tx_path: Base64,
    data_path: Base64,
    chunk: Base64,
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(run())
}

async fn run() {
    let txid = "690t_L2ALtdT8mFvfKmO_u5zGel_x3EtKcKTyo2x6JY";

    let offset = reqwest::get(format!("https://arweave.net/tx/{}/offset", txid))
        .await
        .unwrap()
        .json::<Offset>()
        .await
        .unwrap();

    println!("{:?}", offset);

    let chunk = reqwest::get(format!("https://arweave.net/chunk/{}", offset.offset))
        .await
        .unwrap()
        .json::<RawChunk>()
        .await
        .unwrap();

    println!("{:?}", chunk);
}
