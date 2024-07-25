use clap::Parser;
use core::panic;
use tokio;
use tokio::io::{AsyncWriteExt, BufWriter};

const FIRST_LINE_PARSING: &str = "#EXTINF";

/// CLI m3u8 Downloader
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of m3u8 file (.txt)
    #[arg(short, long)]
    source: String,

    /// Path where the vieo should be saved
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let urls = match get_urls_from_file(args.source.as_str()) {
        Ok(urls) => urls,
        Err(e) => panic!("Failed to parse videos.\nCause: {e}"),
    };
    let stream_vec = match create_vector_of_streams(&urls).await {
        Ok(stream_vec) => stream_vec,
        Err(e) => panic!("Failed getting stream from the http request.\nCause: {e}"),
    };
    match save_stream_to_file(args.output.as_str(), &stream_vec).await {
        Ok(_) => println!("Successfully saved stream to file {}", args.output.as_str()),
        Err(e) => println!("Failed to write stream to file.\nCause: {e}"),
    };
}

fn get_urls_from_file(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(path)?.replace(",", "");
    let parse_content = file_content.lines().skip(5).collect::<Vec<_>>();
    if parse_content
        .get(0)
        .expect("Too few lines in m3u8 file")
        .get(0..7)
        .unwrap_or("")
        != FIRST_LINE_PARSING
    {
        Err("Missing #EXTINF line to begin parsing")?
    }

    Ok(parse_content[1..]
        .iter()
        .map(|s| s.to_string())
        .step_by(2)
        .collect::<Vec<String>>())
}

async fn create_vector_of_streams(urls: &Vec<String>) -> Result<Vec<u8>, reqwest::Error> {
    let mut stream_vec: Vec<u8> = vec![];
    let client = reqwest::Client::new();
    let urls_len = urls.len();

    for (index, url) in urls.iter().enumerate() {
        println!("Saving url stream... [{index}/{urls_len}]");
        let stream = client.get(url.to_string()).send().await?.bytes().await?;
        stream_vec.append(&mut stream.as_ref().to_vec());
    }

    Ok(stream_vec)
}

async fn save_stream_to_file(path: &str, stream_vec: &Vec<u8>) -> Result<(), tokio::io::Error> {
    let file = tokio::fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    println!("Writing to file...");
    writer.write_all(stream_vec.as_ref()).await?;

    Ok(())
}
