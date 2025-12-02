use flate2::read::GzDecoder;
use reqwest;
use serde_json::Value;
use std::io::Read;

#[tokio::test]
async fn successfully_fetch_and_parse_ppvsu_api() {
    let client = reqwest::Client::builder()
        .build()
        .expect("Failed to build HTTP client");

    let response = match client
        .get("https://api.ppvs.su/api/streams")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:144.0) Gecko/20100101 Firefox/144.0",
        )
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Referer", "https://api.ppvs.su/api/streams/")
        .header("Origin", "https://api.ppvs.su/api/streams")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Warning: Could not reach ppvs.su API");
            eprintln!(" Error: {}", e);
            eprintln!("  Run with: cargo test successfully_fetch_and_parse_ppvsu_api -- --ignored");
            return;
        }
    };

    println!("Response status: {}", response.status());
    println!("Response headers: {:?}", response.headers());

    assert!(
        response.status().is_success(),
        "Expected successful response, got: {}",
        response.status()
    );

    let body_bytes = response
        .bytes()
        .await
        .expect("Failed to read response body");

    println!("Response body length: {} bytes", body_bytes.len());
    println!(
        "First two bytes (gzip magic): {:02x} {:02x}",
        body_bytes[0], body_bytes[1]
    );

    let decoded_text = if body_bytes.len() > 2 && body_bytes[0] == 0x1f && body_bytes[1] == 0x8b {
        println!("Response is gzip compressed, decompressing...");
        let mut decoder = GzDecoder::new(&body_bytes[..]);
        let mut decompressed = String::new();
        decoder
            .read_to_string(&mut decompressed)
            .expect("Failed to decompress gzip");
        decompressed
    } else {
        String::from_utf8(body_bytes.to_vec()).expect("Failed to convert to UTF-8")
    };

    println!(
        "Decoded response (first 200 chars): {}",
        &decoded_text.chars().take(200).collect::<String>()
    );

    let json: Value = serde_json::from_str(&decoded_text).expect("Failed to parse JSON response");

    println!(
        "Response body (JSON): {}",
        serde_json::to_string_pretty(&json).unwrap()
    );

    assert!(
        json.get("success").is_some(),
        "Response missing 'success' field"
    );

    assert_eq!(
        json["success"].as_bool(),
        Some(true),
        "Expected success to be true"
    );

    assert!(
        json.get("streams").is_some(),
        "Response missing 'streams' field"
    );

    let streams = json["streams"]
        .as_array()
        .expect("streams should be an array");

    assert!(!streams.is_empty(), "Expected at least one stream category");

    println!("Successfully fetched {} stream categories", streams.len());
    println!("API response structure validated");
}
