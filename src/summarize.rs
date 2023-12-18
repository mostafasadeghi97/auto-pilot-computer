use crate::{
    action::send_message_to_openai,
    constants::MAX_TOKENS,
    parsers::format_summary_prompt,
    screen::capture_screen_with_cursor,
    types::{
        ImageMessage, ImageMessageContent, ImageUrl, Message, OpenAIRequest, Role, TextMessage,
    },
};
use base64::{engine::general_purpose, Engine as _};
use std::{error::Error, fs, path::Path};

pub async fn summarize(
    messages: &mut Vec<Message>,
    objective: &str,
) -> Result<String, Box<dyn Error>> {
    let screenshots_dir = "screenshots";
    if !Path::new(screenshots_dir).exists() {
        fs::create_dir(screenshots_dir)?;
    }

    let screenshot_filename = format!("{}/summary_screenshot.png", screenshots_dir);

    capture_screen_with_cursor(&screenshot_filename)?;

    let img_file = fs::read(&screenshot_filename)?;
    let img_base64 = general_purpose::STANDARD.encode(&img_file);

    let summary_prompt = format_summary_prompt(objective);

    let vision_message = Message::ImageMessage(ImageMessage {
        role: Role::User,
        content: vec![
            ImageMessageContent::Text {
                text: summary_prompt,
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
        content: "summary_screenshot.png".to_string(),
    }));

    messages.push(Message::TextMessage(TextMessage {
        role: Role::Assistant,
        content: content.to_string(),
    }));

    Ok(content)
}
