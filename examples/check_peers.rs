use futures::future::try_join_all;
use reqwest;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(run())
}
async fn run() {
    let txid = "690t_L2ALtdT8mFvfKmO_u5zGel_x3EtKcKTyo2x6JY";

    let peers = reqwest::get("https://arweave.net/peers")
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
        .unwrap();

    let peers_chunks: Vec<&[String]> = peers.chunks(25).collect();
    for chunk in peers_chunks {
        let good_resp = try_join_all(chunk.iter().map(|p| {
            reqwest::Client::new()
                .get(format!("http://{}/tx/{}/data", p, txid))
                .timeout(std::time::Duration::from_secs(5))
                .send()
        }))
        .await;

        if let Ok(good_resp) = good_resp {
            let good_resp = good_resp
                .into_iter()
                .filter(|r| r.status() == reqwest::StatusCode::OK)
                .collect::<Vec<reqwest::Response>>();

            for resp in good_resp {
                let headers = resp.headers().clone();
                headers.get("content-length");
                {
                    println!(
                        "{}: {:?}",
                        resp.url(),
                        headers.get("content-length").unwrap()
                    );
                }
            }
        }
    }
}
