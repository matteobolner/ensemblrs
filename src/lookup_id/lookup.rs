use core::panic;

use crate::structs::structs::EnsemblFeature;
use reqwest::header::CONTENT_TYPE;
//#[tokio::main]
pub async fn lookup_id(
    id: &str,
    expand: &str,
) -> Result<EnsemblFeature, Box<dyn std::error::Error>> {
    //let url = "https://rest.ensembl.org/lookup/id/ENSG00000157764?"; //;expand=1";
    let expand_code = match expand {
        "all" | "a" | "canonical" | "c" => 1,
        "false" | "f" => 0,
        _ => panic!("Invalide option for expand: {expand} ; see help for possible values"),
    };
    let url = format!(
        "https://rest.ensembl.org/lookup/id/{}?;expand={}",
        &id, expand_code as i32
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    if resp.status().is_success() {
        let feature: EnsemblFeature = resp.json().await?;
        //let bed = feature.to_bed(name="AAA")
        Ok(feature)
    } else {
        // Print the error status and message
        panic!("Error: {}", resp.status());
    }
}
