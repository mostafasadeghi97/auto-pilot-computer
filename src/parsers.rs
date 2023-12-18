use crate::{
    prompts::{SUMMARY_PROMPT, VISION_PROMPT},
    types::{Message, Role, TextMessage},
};
use regex::Regex;

pub enum ActionType {
    Search,
    Type,
    Click,
    Unknown,
    Done,
}

impl ActionType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "DONE" => ActionType::Done,
            s if s.starts_with("CLICK") => ActionType::Click,
            s if s.starts_with("TYPE") => ActionType::Type,
            s if s.starts_with("SEARCH") => ActionType::Search,
            _ => ActionType::Unknown,
        }
    }
}

pub fn format_summary_prompt(objective: &str) -> String {
    format!("{}", SUMMARY_PROMPT.replace("{objective}", objective))
}

pub fn format_vision_prompt(objective: &str, previous_action: &str) -> String {
    let previous_action_formatted = if !previous_action.is_empty() {
        format!("Here was the previous action you took: {}", previous_action)
    } else {
        String::new()
    };

    VISION_PROMPT
        .replace("{objective}", objective)
        .replace("{previous_action}", &previous_action_formatted)
}

pub fn parse_openai_response(response: &str) -> Result<(String, String), &'static str> {
    let cleaned_response = response.trim_matches(&['"', '\\'] as &[_]);

    let action_type = ActionType::from_str(cleaned_response);

    match action_type {
        ActionType::Done => Ok(("DONE".to_string(), "".to_string())),
        ActionType::Click => parse_action_response(
            cleaned_response,
            r"CLICK \{\{(.+)\}\}",
            "CLICK",
            &['\\'] as &[_],
            true,
        ),
        ActionType::Type => parse_action_response(
            cleaned_response,
            r#"TYPE\s(.+)"#,
            "TYPE",
            &['\\', '\"'] as &[_],
            false,
        ),
        ActionType::Search => parse_action_response(
            cleaned_response,
            r#"SEARCH\s(.+)"#,
            "SEARCH",
            &['\\', '\"'] as &[_],
            false,
        ),
        ActionType::Unknown => Ok(("UNKNOWN".to_string(), cleaned_response.to_string())),
    }
}

fn parse_action_response(
    response: &str,
    pattern: &str,
    action: &str,
    trim_chars: &[char],
    wrap_in_braces: bool,
) -> Result<(String, String), &'static str> {
    let re = Regex::new(pattern).unwrap();

    re.captures(response)
        .and_then(|caps| caps.get(1))
        .map(|match_| {
            (
                action.to_string(),
                if wrap_in_braces {
                    format!("{{{}}}", match_.as_str().trim_matches(trim_chars))
                } else {
                    match_.as_str().trim_matches(trim_chars).to_string()
                },
            )
        })
        .ok_or("Regex parsing failed")
}

pub fn convert_percent_to_decimal(percent_str: &str) -> Result<f32, std::num::ParseFloatError> {
    let percent_str = percent_str.trim_matches(&['"', '%'] as &[_]);
    percent_str.parse::<f32>().map(|num| num / 100.0)
}

pub fn convert_string_to_json(s: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(s)
}

pub fn get_last_assistant_message(messages: &mut Vec<Message>) -> String {
    messages
        .iter()
        .rev()
        .find_map(|msg| match msg {
            Message::TextMessage(TextMessage { role, content }) if *role == Role::Assistant => {
                Some(content.clone())
            }
            _ => None,
        })
        .unwrap_or_else(String::new)
}
