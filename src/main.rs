use clap::{Parser, Subcommand};
use serde_json::{from_str, json, Value};

const ENDPOINT: &str = "https://api.openai.com/v1/completions";

#[derive(Parser, Debug)]
struct WriteArgs {
    #[arg(index = 1)]
    prompt: String,
}

#[derive(Subcommand, Debug)]
enum Action {
    Write(WriteArgs),
    Refactor(WriteArgs),
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,

    #[arg(short, long)]
    lang: String,

    #[arg(long, env = "OPENAI_API_KEY")]
    api_key: String,

    #[arg(short, long, default_value_t = String::from("text-davinci-003"))]
    model: String,

    #[arg(short = 't', long, default_value_t = 1024)]
    max_tokens: i32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let prompt = match args.action {
        Action::Write(WriteArgs { prompt }) => format!(
            "write {}, {}. Don't write explanations or anything else other than code",
            args.lang, prompt
        ),
        Action::Refactor(WriteArgs { prompt }) => {
            format!("refactor this {} code: ```{}```", args.lang, prompt)
        }
    };

    let body = json!({
        "model": args.model,
        "prompt": prompt,
        "max_tokens": args.max_tokens
    });

    let resp: String = ureq::post(ENDPOINT)
        .set("Authorization", format!("Bearer {}", args.api_key).as_str())
        .send_json(&body)?
        .into_string()?;

    let value: Value = from_str(&resp)?;
    let choice = &value["choices"][0]["text"];
    let code = snailquote::unescape(&choice.to_string()).unwrap();

    println!("{code}");

    Ok(())
}
