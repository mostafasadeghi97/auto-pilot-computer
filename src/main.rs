use auto_pilot::{
    action::get_next_action_from_openai,
    initialize::{get_user_objective, initialize_messages},
    operations::{keyboard_type, mouse_click, search},
    parsers::{convert_string_to_json, parse_openai_response, ActionType},
    summarize::summarize,
    types::Message,
};
use clap::Parser;
use colored::Colorize;
use std::{env, error::Error};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mostafa Sadeghi",
    about = "Auto Pilot Computer",
    long_about = "This is a tool that uses GPT4 Vision to operate your computer."
)]
struct Cli {
    /// The objective you want to achieve with the computer
    #[clap(short, long)]
    objective: Option<String>,

    /// The grid interval to use when capturing the screen. Default is 300. The smaller the number, more number of lines will be drawn. (closer to pixel level)
    #[clap(short, long, default_value = "300")]
    grid_interval: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Cli = Cli::parse();

    let objective = match opts.objective {
        Some(objective) => objective,
        None => get_user_objective()?,
    };

    let grid_interval = opts.grid_interval;

    env::set_var("RUST_BACKTRACE", "1");
    let mut messages = initialize_messages(&objective);

    let result = run_auto_pilot(&mut messages, &objective, grid_interval).await;
    if let Err(e) = result {
        eprintln!("Error occurred: {}", e);
    }

    Ok(())
}

async fn run_auto_pilot(
    messages: &mut Vec<Message>,
    objective: &str,
    grid_interval: i32,
) -> Result<(), Box<dyn Error>> {
    let mut loop_count = 0;
    loop {
        let response = get_next_action_from_openai(messages, objective, grid_interval).await?;

        let (action_type, action_detail) = parse_openai_response(&response)?;

        let function_response = match ActionType::from_str(&action_type) {
            ActionType::Search => search(&action_detail),
            ActionType::Type => keyboard_type(&action_detail),
            ActionType::Click => {
                let click_detail = convert_string_to_json(&action_detail)?;
                mouse_click(&click_detail)
            }
            ActionType::Unknown => {
                eprintln!("Something went wrong :(");
                eprintln!("AI response: {}", response);
                eprintln!("action_type: {}", action_type);
                eprintln!("action_detail: {}", action_detail);
                break;
            }
            ActionType::Done => {
                let summary = summarize(messages, objective).await?;
                println!("{}: {}", "Summary".bright_magenta(), summary);
                break;
            }
        };

        println!(
            "{} {} {} {} {}",
            "[Auto-Pilot-Computer]".blue(),
            "[Act]".bright_magenta(),
            action_type,
            "COMPLETE".green(),
            function_response
        );

        loop_count += 1;
        if loop_count > 15 {
            break;
        }
    }

    Ok(())
}
