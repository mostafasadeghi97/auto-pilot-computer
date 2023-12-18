use crate::{
    prompts::ASSISTANT_MESSAGE,
    types::{Message, Role, TextMessage},
};
use colored::Colorize;
use std::io;

pub fn get_user_objective() -> Result<String, io::Error> {
    println!("{}", ASSISTANT_MESSAGE.blue());
    let mut objective = String::new();
    io::stdin().read_line(&mut objective)?;
    Ok(objective.trim().to_string())
}

pub fn initialize_messages(objective: &str) -> Vec<Message> {
    vec![
        Message::TextMessage(TextMessage {
            role: Role::Assistant,
            content: ASSISTANT_MESSAGE.to_string(),
        }),
        Message::TextMessage(TextMessage {
            role: Role::User,
            content: format!("Objective: {}", objective),
        }),
    ]
}
