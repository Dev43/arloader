use arloader::Arweave;
use std::{path::PathBuf, time::Instant};

fn is_valid_file_path(path_str: &str) -> Result<(), String> {
    match path_str.parse::<PathBuf>() {
        Ok(p) => {
            if p.exists() {
                if p.is_file() {
                    Ok(())
                } else {
                    return Err("Path is not file.".to_string());
                }
            } else {
                return Err("Path does not exist.".to_string());
            }
        }
        Err(_) => return Err("Not a valid path.".to_string()),
    }
}
fn main() -> Result<(), String> {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(run())
}
async fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let arweave = Arweave::default();

    println!("{:?}", args);

    if args.len() != 2 {
        return Err("must provide exactly one existing file path argument".to_string());
    }
    is_valid_file_path(&args[1]).unwrap();

    let bytes = std::fs::read(args[1].clone()).unwrap();

    let start = Instant::now();
    arweave
        .create_transaction(bytes, None, None, (0, 0), true)
        .await
        .unwrap();
    let duration = start.elapsed();

    println!("Transaction created in: {:?}", duration);

    Ok(())
}
