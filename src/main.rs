use clap::{Parser, Subcommand};
use serde_json::{from_str, json, Value};

#[derive(Parser, Debug, Clone)]
struct WriteArgs {
    #[arg(index = 1)]
    prompt: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Write(WriteArgs),
    Refactor(WriteArgs),
    Explain(WriteArgs),
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,

    #[arg(short, long)]
    lang: String,

    #[arg(long, env = "OPENAI_API_KEY")]
    api_key: String,

    #[arg(short, long, default_value_t = String::from("text-davinci-001"))]
    model: String,

    #[arg(short = 't', long, default_value_t = 1024)]
    max_tokens: i32,

    #[arg(long, default_value_t = String::from("https://api.openai.com/v1/completions"), env = "OPENAI_ENDPOINT")]
    endpoint: String,
}

pub fn into_comment(text: String, lang: String) -> anyhow::Result<String> {
    match lang.as_str() {
        "rust" | "c" | "javascript" | "typescript" | "solidity" => {
            let regex = regex::Regex::new(r"(?m)^")?;
            Ok(regex.replace_all(&text, "// ").to_string())
        }
        "dockerfile" | "bash" | "zsh" | "sh" | "python" | "ruby" => {
            let regex = regex::Regex::new(r"(?m)^")?;
            Ok(regex.replace_all(&text, "# ").to_string())
        }
        "lua" | "sql" => {
            let regex = regex::Regex::new(r"(?m)^")?;
            Ok(regex.replace_all(&text, "-- ").to_string())
        }
        _ => Ok(text),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let prompt = match args.action.clone() {
        Action::Write(WriteArgs { prompt }) => format!(
            "write {}, {}. Don't write explanations or anything else other than code",
            args.lang, prompt
        ),
        Action::Refactor(WriteArgs { prompt }) => {
            format!("refactor this {} code: ```\n{}```", args.lang, prompt)
        }
        Action::Explain(WriteArgs { prompt }) => {
            format!("explain this {} code: ```\n{}```", args.lang, prompt)
        }
    };

    let body = json!({
        "model": args.model,
        "prompt": prompt,
        "max_tokens": args.max_tokens
    });

    let resp: String = ureq::post(&args.endpoint)
        .set("Authorization", format!("Bearer {}", args.api_key).as_str())
        .send_json(&body)?
        .into_string()?;

    let value: Value = from_str(&resp)?;
    let choice = &value["choices"][0]["text"];
    let mut code = snailquote::unescape(&choice.to_string())
        .unwrap()
        .trim()
        .to_string();

    if let Action::Explain(_) = args.action {
        code = textwrap::fill(&code, 80);
        code = into_comment(code, args.lang)?;
    }

    println!("{code}");

    Ok(())
}
