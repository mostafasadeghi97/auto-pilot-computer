use crate::constants::{MAX_TOKENS, OPENAI_ENDPOINT};
use crate::parsers::{format_vision_prompt, get_last_assistant_message};
use crate::screen::{add_grid_to_image, capture_screen_with_cursor};
use crate::types::{
    ImageMessage, ImageMessageContent, ImageUrl, Message, OpenAIRequest, Role, TextMessage,
};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde_json::Value;
use std::{env, fs, path::Path, thread, time::Duration};

pub async fn get_next_action_from_openai(
    messages: &mut Vec<Message>,
    objective: &str,
    grid_interval: i32,
) -> Result<String, String> {
    thread::sleep(Duration::from_secs(1));

    let screenshots_dir = "screenshots";
    if !Path::new(screenshots_dir).exists() {
        fs::create_dir(screenshots_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let screenshot_filename = format!("{}/screenshot.png", screenshots_dir);
    capture_screen_with_cursor(&screenshot_filename)
        .map_err(|e| format!("Error capturing screen: {}", e))?;

    let new_screenshot_filename = format!("{}/screenshot_with_grid.png", screenshots_dir);
    add_grid_to_image(
        &screenshot_filename,
        &new_screenshot_filename,
        grid_interval,
    )
    .map_err(|e| format!("Error adding grid to image: {}", e))?;

    thread::sleep(Duration::from_secs(1));

    let img_file = fs::read(&new_screenshot_filename)
        .map_err(|e| format!("Error reading screenshot file: {}", e))?;
    let img_base64 = general_purpose::STANDARD.encode(&img_file);

    let mut previous_action = get_last_assistant_message(messages);
    let vision_prompt = format_vision_prompt(objective, &mut previous_action);

    let vision_message = Message::ImageMessage(ImageMessage {
        role: Role::User,
        content: vec![
            ImageMessageContent::Text {
                text: vision_prompt,
            },
            ImageMessageContent::ImageUrl {
                image_url: ImageUrl {
                    url: format!("data:image/jpeg;base64,{}", img_base64),
                },
            },
        ],
    });

    let mut messages_clone = messages.clone();
    messages_clone.push(vision_message);

    let payload = OpenAIRequest {
        model: "gpt-4-vision-preview".to_string(),
        messages: messages_clone,
        max_tokens: MAX_TOKENS,
    };

    let content = send_message_to_openai(payload)
        .await
        .map_err(|e| format!("Error sending message to OpenAI: {}", e))?;

    messages.push(Message::TextMessage(TextMessage {
        role: Role::User,
        content: "screenshot.png".to_string(),
    }));

    messages.push(Message::TextMessage(TextMessage {
        role: Role::Assistant,
        content: content.to_string(),
    }));

    Ok(content.replace("\\", ""))
}

pub async fn send_message_to_openai(payload: OpenAIRequest) -> Result<String, String> {
    let client = Client::new();

    let openai_api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY not found in environment".to_string())?;

    let seralized_payload = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize payload: {}", e))?;

    let response: Value = client
        .post(OPENAI_ENDPOINT)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .body(seralized_payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let content = response["choices"][0]["message"]["content"].to_string();

    Ok(content)
}
