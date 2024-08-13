use reqwest::{header::CONTENT_TYPE, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GeneInfo {
    display_name: String,
    biotype: String,
    logic_name: String,
    object_type: String,
    version: u8,
    seq_region_name: String,
    species: String,
    end: u64,
    canonical_transcript: String,
    db_type: String,
    start: i64,
    assembly_name: String,
    source: String,
    strand: i8,
    id: String,
    description: String,
}

// Define the async function `test`.
async fn test() -> Result<GeneInfo, anyhow::Error> {
    let url = "https://rest.ensembl.org/lookup/id/ENSG00000157764?";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let resp_text = resp.text().await?;
    let gene_info = serde_json::from_str(&resp_text)?;

    Ok(gene_info)
}

// The main function, now asynchronous.
#[tokio::main]
async fn main() {
    // Call the `test` function and await its result.
    match test().await {
        Ok(gene_info) => {
            // Print the successful result.
            println!("Response: {:#?}", gene_info);
        }
        Err(e) => {
            // Handle the error.
            println!("Error occurred: {:#?}", e);
        }
    }
}
