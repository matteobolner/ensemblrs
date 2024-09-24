use reqwest::header::CONTENT_TYPE;

pub mod structs;

/// Simple program to download genome information and sequence from NCBI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Ensembl ID
    #[arg(required = true, short, long)]
    id: String,
    /// Expand
    #[arg(short, long, default_value = false)]
    expand: bool,
}


add parsing of ensembl id to struct type, to avoid two get calls

#[tokio::main]
async fn lookup_id(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://rest.ensembl.org/lookup/id/ENSG00000157764?"; //;expand=1";

    let url = formatln!(
        "https://rest.ensembl.org/lookup/id/{}?;expand={}",
        &args.id,
        &args.expand as i32
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    if resp.status().is_success() {
        let gene: Gene = resp.json().await?;
        println!("{:#?}", gene);
    } else {
        // Print the error status and message
        println!("Error: {}", resp.status());
    }
    Ok(())
}
