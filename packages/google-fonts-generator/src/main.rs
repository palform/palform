use std::{fs::File, io::Write};

use serde::Deserialize;

#[derive(Deserialize)]
struct GFWebfontList {
    items: Vec<GFWebfont>,
}

#[derive(Deserialize, Clone)]
struct GFWebfont {
    family: String,
}

#[tokio::main]
async fn main() {
    let google_fonts_api_key = std::env::var("PAL_GOOGLE_FONTS_API_KEY")
        .expect("PAL_GOOGLE_FONTS_API_KEY environment variable not found at runtime");

    let mut file = File::create("packages/backend/src/static/google_fonts.json")
        .expect("open/create output file");

    let resp = reqwest::get(format!(
        "https://www.googleapis.com/webfonts/v1/webfonts?key={}",
        google_fonts_api_key,
    ))
    .await
    .expect("make request")
    .json::<GFWebfontList>()
    .await
    .expect("decode response");

    let family_names: Vec<String> = resp.items.iter().map(|f| f.clone().family).collect();
    let json_string = serde_json::to_string(&family_names).expect("serialize vec");

    file.write_all(json_string.as_bytes())
        .expect("write output to file");

    println!("all done :)");
}
