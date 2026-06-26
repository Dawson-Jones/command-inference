use serde::{Deserialize, Serialize};
use std::env;
use std::process;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

const SYSTEM_PROMPT: &str = "\
You are a shell command fixer. The user will give you an approximate or broken shell command. \
Return ONLY the corrected command, nothing else — no explanation, no markdown, no code fence. \
If the command is already correct, return it as-is.";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cmdi <approximate command>");
        process::exit(1);
    }

    let input = args.join(" ");

    let api_key = env::var("CMDI_KEY").unwrap_or_else(|_| {
        eprintln!("Error: CMDI_KEY environment variable not set");
        process::exit(1);
    });

    let request = ChatRequest {
        model: "deepseek-v4-flash".into(),
        messages: vec![
            Message { role: "system".into(), content: SYSTEM_PROMPT.into() },
            Message { role: "user".into(), content: input },
        ],
        temperature: 0.0,
    };

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request)
        .send();

    match resp {
        Ok(r) => {
            if !r.status().is_success() {
                eprintln!("API error: {} {}", r.status(), r.text().unwrap_or_default());
                process::exit(1);
            }
            match r.json::<ChatResponse>() {
                Ok(chat) => {
                    if let Some(choice) = chat.choices.first() {
                        println!("{}", choice.message.content.trim());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse response: {e}");
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
