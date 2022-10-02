use arloader::{commands::CommandResult, Arweave};
use std::sync::Arc;

fn main() -> CommandResult {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(run())
}
async fn run() -> CommandResult {
    let arweave = Arc::new(Arweave::default());

    let mut price_futures = Vec::new();
    for m in 2..6 {
        let arweave = arweave.clone();

        price_futures.push(tokio::task::spawn(async move {
            arweave.get_price_terms(m as f32).await
        }));
    }

    let results = futures::future::join_all(price_futures).await;
    for result in results {
        println!("{:?}", result.unwrap())
    }

    Ok(())
}
