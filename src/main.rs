use bat::PrettyPrinter;
use clap::Parser;
use serde_json::{from_str, json, Value};

const ENDPOINT: &str = "https://api.openai.com/v1/completions";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    lang: String,
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long, default_value_t = false)]
    color: bool,
    #[arg(long, env = "OPENAI_API_KEY")]
    api_key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let body = json!({
        "model": "text-davinci-003",
        "prompt": [format!("write {} code, {}", args.lang, args.prompt)],
        "max_tokens": 1024
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(ENDPOINT)
        .bearer_auth(args.api_key)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    let value: Value = from_str(&resp)?;
    let choice = &value["choices"][0]["text"];
    let code = snailquote::unescape(&choice.to_string()).unwrap();

    if args.color {
        PrettyPrinter::new()
            .input_from_bytes(code.as_bytes())
            .language(&args.lang)
            .print()
            .unwrap();
    } else {
        println!("{code}");
    }

    Ok(())
}
