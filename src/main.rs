use bat::PrettyPrinter;
use clap::{Parser, Subcommand};
use serde_json::{from_str, json, Value};

const ENDPOINT: &str = "https://api.openai.com/v1/completions";

#[derive(Subcommand, Debug)]
enum Action {
    Write,
    Refactor,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,

    #[arg(short, long)]
    lang: String,

    #[arg(short, long)]
    prompt: String,

    #[arg(short, long, default_value_t = false)]
    color: bool,

    #[arg(long, env = "OPENAI_API_KEY")]
    api_key: String,

    #[arg(short, long, default_value_t = String::from("text-davinci-003"))]
    model: String,

    #[arg(short = 't', long, default_value_t = 1024)]
    max_tokens: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let body = match args.action {
        Action::Write => json!({
            "model": args.model,
            "prompt": [format!("write {} code, {}", args.lang, args.prompt)],
            "max_tokens": args.max_tokens
        }),
        Action::Refactor => json!({
            "model": args.model,
            "prompt": [format!("refactor this {} code: ```{}```", args.lang, args.prompt)],
            "max_tokens": args.max_tokens
        }),
    };

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
