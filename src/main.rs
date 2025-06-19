use anyhow::Result;
use clap::Parser;

mod providers;

#[derive(Parser)]
#[command(name = "clm")]
#[command(about = "Command Line LLM tool")]
struct Cli {
    /// The question or prompt to send to the AI
    #[arg(trailing_var_arg = true)]
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.prompt.is_empty() {
        eprintln!("Error: Please provide a prompt");
        std::process::exit(1);
    }

    let prompt = cli.prompt.join(" ");

    let provider = providers::get_provider()?;

    let system_message = String::from(
        "You are a helpful assistant. You will receive a prompt and you will respond with a short, concise answer. If you respond with a code block, please format it using markdown syntax. Before the code block, please include a brief explanation of what the code does. Separate the explanation from the code block using --- dashes.",
    );

    let complete_prompt = format!("{} --- {}", system_message, prompt);

    match provider.query(&complete_prompt).await {
        Ok(response) => {
            println!("{}", response.content);

            let tokens_text = if let Some(tokens) = response.tokens_used {
                format!("Tokens: {}", tokens)
            } else {
                "Tokens: N/A".to_string()
            };

            let duration_text = format!("Time: {:.2}s", response.duration.as_secs_f64());

            let provider_name = std::env::var("CLM_PROVIDER")
                .unwrap_or_else(|_| "google".to_string())
                .to_lowercase();

            println!(
                "\n[{} | {} | {}]",
                tokens_text, duration_text, provider_name
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
