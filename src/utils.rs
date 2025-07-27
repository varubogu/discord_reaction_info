use anyhow::{anyhow, Result};
use regex::Regex;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::{marker::{ChannelMarker, MessageMarker}, Id};

/// Parse a message identifier (URL or ID) into channel_id and message_id
///
/// # Arguments
///
/// * `message_identifier` - A string that is either a message URL or a message ID
///
/// # Returns
///
/// * `Result<(Id<ChannelMarker>, Id<MessageMarker>)>` - A tuple of channel_id and message_id
#[allow(dead_code)]
pub fn parse_message_identifier(message_identifier: &str) -> Result<(Id<ChannelMarker>, Id<MessageMarker>)> {
    // Check if the message_identifier is a numeric string (message ID)
    if message_identifier.chars().all(|c| c.is_digit(10)) {
        return Err(anyhow!("When providing just a message ID, you must also specify the channel ID"));
    }

    // Try to parse as a Discord message URL
    let re = Regex::new(r"https://discord\.com/channels/(?:\d+)/(\d+)/(\d+)")?;
    if let Some(captures) = re.captures(message_identifier) {
        let channel_id_str = captures.get(1).unwrap().as_str();
        let message_id_str = captures.get(2).unwrap().as_str();

        let channel_id = Id::new(channel_id_str.parse()?);
        let message_id = Id::new(message_id_str.parse()?);

        return Ok((channel_id, message_id));
    }

    Err(anyhow!("Invalid message identifier format. Please provide a valid Discord message URL"))
}

/// Create an error response
///
/// # Arguments
///
/// * `error_message` - The error message to display
///
/// # Returns
///
/// * `InteractionResponse` - The error response
#[allow(dead_code)]
pub fn create_error_response(error_message: &str) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(twilight_model::http::interaction::InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: Some(format!("Error: {}", error_message)),
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: Some(false),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_parse_message_identifier_valid_url() {
        let url = "https://discord.com/channels/123456789012345678/234567890123456789/345678901234567890";
        let result = parse_message_identifier(url);

        assert!(result.is_ok());
        let (channel_id, message_id) = result.unwrap();
        assert_eq!(channel_id.get(), 234567890123456789);
        assert_eq!(message_id.get(), 345678901234567890);
    }

    #[test]
    fn test_parse_message_identifier_invalid_url() {
        let url = "https://discord.com/invalid/url";
        let result = parse_message_identifier(url);

        assert!(result.is_err());
        assert_matches!(result.unwrap_err().to_string().as_str(), 
            s if s.contains("Invalid message identifier format"));
    }

    #[test]
    fn test_parse_message_identifier_numeric_id() {
        let id = "123456789012345678";
        let result = parse_message_identifier(id);

        assert!(result.is_err());
        assert_matches!(result.unwrap_err().to_string().as_str(), 
            s if s.contains("When providing just a message ID"));
    }

    #[test]
    fn test_create_error_response() {
        let error_message = "Test error message";
        let response = create_error_response(error_message);

        assert_eq!(response.kind, InteractionResponseType::ChannelMessageWithSource);
        assert!(response.data.is_some());

        let data = response.data.unwrap();
        assert!(data.content.is_some());
        assert_eq!(data.content.unwrap(), format!("Error: {}", error_message));
    }
}