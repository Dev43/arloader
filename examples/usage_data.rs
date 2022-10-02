use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

async fn write_data() {
    let query = r#"query Transactions ($arg1: String) {
        transactions(
          after: $arg1
          first:1000
          tags: [
          {
            name: "User-Agent",
            values: [
              "arloader/0.1.18",
              "arloader/0.1.19",
              "arloader/0.1.20",
              "arloader/0.1.21",
              "arloader/0.1.22",
              "arloader/0.1.23",
              "arloader/0.1.23",
              "arloader/0.1.24",
              "arloader/0.1.25",
              "arloader/0.1.26",
              "arloader/0.1.27",
              "arloader/0.1.28",
              "arloader/0.1.29",
              "arloader/0.1.30",
              "arloader/0.1.31",
              "arloader/0.1.32",
              "arloader/0.1.33",
              "arloader/0.1.34",
              "arloader/0.1.35",
              "arloader/0.1.36",
              "arloader/0.1.37",
              "arloader/0.1.38",
              "arloader/0.1.39",
              "arloader/0.1.40",
              "arloader/0.1.41",
              "arloader/0.1.42",
              "arloader/0.1.43",
              "arloader/0.1.44",
              "arloader/0.1.45",
              "arloader/0.1.46",
              "arloader/0.1.47",
              "arloader/0.1.48",
              "arloader/0.1.49",
              "arloader/0.1.50",
              "arloader/0.1.51",
            ]
          },
          # {
          #   name: "Content-Type",
          #   values: ["image/png"]
          # },
        ],
        sort: HEIGHT_DESC)
      {
        
        edges {
          cursor  
                node {
                    id,
                  owner {
                    address
                  },
                  block {
                    timestamp,
                    height
                  },
                  tags {
                    name,
                    value
                  },
                  data {
                    size,
                    type
                  }
                  bundledIn {
                    id
                  }
                  fee {
                    winston
                  }
                  quantity {
                    winston
                  }
                }
            }
        }
    }"#;

    let mut values: Vec<Value> = Vec::new();
    let resp = reqwest::Client::new()
        .post("https://arweave.net/graphql")
        .json(&json!({ "query": query, "operationName": "Transactions", "variables": json!({"arg1": ""})}))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    fn get_record(value: &Value) -> Value {
        let obj = value.as_object().unwrap();
        json!({
          "cursor": obj["cursor"],
          "id": obj["node"]["id"],
          "bundled_in": obj["node"]["bundledIn"],
          "owner": obj["node"]["owner"]["address"],
          "data_size": u64::from_str(obj["node"]["data"]["size"].as_str().unwrap()).unwrap(),
          "fee": u64::from_str(obj["node"]["fee"]["winston"].as_str().unwrap()).unwrap(),
          "quantity": u64::from_str(obj["node"]["quantity"]["winston"].as_str().unwrap()).unwrap()
        })
    }

    let mut transactions: Vec<Value> = resp.as_object().unwrap()["data"]["transactions"]["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(get_record)
        .collect();

    while transactions.len() > 0 {
        values.append(&mut transactions);
        let cursor = values.last().unwrap().as_object().unwrap()["cursor"]
            .as_str()
            .unwrap();
        let resp = reqwest::Client::new()
            .post("https://arweave.net/graphql")
            .json(&json!({ "query": query, "operationName": "Transactions", "variables": json!({"arg1": cursor})}))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        transactions = resp.as_object().unwrap()["data"]["transactions"]["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(get_record)
            .collect();
        println!("{:?}", values.len());
    }
    fs::write("data.json", serde_json::to_string(&values).unwrap()).unwrap();
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(run())
}
async fn run() {
    if !std::path::PathBuf::from("data.json").exists() {
        write_data().await;
    }
    let data = fs::read_to_string("data.json").unwrap();
    let trans: Value = serde_json::from_str(&data).unwrap();

    let owner_count = trans
        .as_array()
        .unwrap()
        .into_iter()
        .fold(HashMap::new(), |mut map, t| {
            let obj = t.as_object().unwrap();
            *map.entry(obj["owner"].as_str().unwrap())
                .or_insert(obj["data_size"].as_u64().unwrap() / 1_000_000) +=
                obj["data_size"].as_u64().unwrap() / 1_000_000;
            map
        });

    for (owner, count) in owner_count.iter() {
        println!("{:<20} {:>10}", owner, count);
    }
}
