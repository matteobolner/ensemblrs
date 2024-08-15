use reqwest::{header::CONTENT_TYPE, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GenomeCoords {
    assembly_name: String,
    seq_region_name: String,
    start: u64,
    end: u64,
    strand: i8,
}

// MetaInfo struct for common metadata fields
#[derive(Debug, Deserialize, Serialize)]
struct MetaInfo {
    db_type: String,
    object_type: String,
    species: String,
    version: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Exon {
    id: String,
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Translation {
    id: String,
    #[serde(rename = "Parent")]
    parent: String,
    start: u64,
    end: u64,
    length: u64,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transcript {
    id: String,
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(rename = "Exon")]
    exons: Vec<Exon>,
    #[serde(rename = "Translation")]
    translation: Translation,
    display_name: String,
    is_canonical: u64,
    length: u64,
    logic_name: String,
    biotype: String,
    #[serde(flatten)]
    meta: MetaInfo,
    #[serde(flatten)]
    coords: GenomeCoords,
    source: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Gene {
    id: String,
    canonical_transcript: String,
    description: String,
    display_name: String,
    logic_name: String,
    biotype: String,
    source: String,
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneAndTranscripts {
    #[serde(flatten)]
    gene: Gene,
    #[serde(rename = "Transcript")]
    transcripts: Vec<Transcript>,
}

// Define the async function `test`.
async fn lookup_id() -> Result<Gene, anyhow::Error> {
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://rest.ensembl.org/lookup/id/ENSSSCG00000031764?;expand=1";

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    if resp.status().is_success() {
        //let resp_text = resp.text().await?;
        //println!("{}", resp_text);
        //let resp_text = resp.json::<serde_json::Value>().await?;
        //println!("{:#?}", resp_text);
        // Parse the response body as JSON
        let gene: GeneAndTranscripts = resp.json().await?;
        println!("{:#?}", gene);
    } else {
        // Print the error status and message
        println!("Error: {}", resp.status());
    }
    Ok(())
}
