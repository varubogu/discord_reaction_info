use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::Reaction;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::{Id, marker::MessageMarker};

use crate::utils::{self, create_error_response};

#[derive(CommandModel, CreateCommand)]
#[command(name = "rinfo", desc = "Get reaction information for a message")]
pub struct RinfoCommand {
    /// Message URL or ID
    message: String,

    /// Users to exclude from the results
    #[command(desc = "Users to exclude from the results")]
    exclude_user: Option<String>,

    /// Reactions to exclude from the results
    #[command(desc = "Reactions to exclude from the results")]
    exclude_reaction: Option<String>,

    /// Include the message author in the results
    #[command(desc = "Include the message author in the results")]
    include_message_user: Option<bool>,

    /// Only show users, not grouped by reaction
    #[command(desc = "Only show users, not grouped by reaction")]
    user_only: Option<bool>,
}

impl RinfoCommand {
    pub async fn handle(
        self,
        http: Arc<HttpClient>,
        interaction: &Interaction,
        _command_data: &CommandData,
    ) -> Result<InteractionResponse> {
        // Parse the message identifier
        let (channel_id, message_id) = match utils::parse_message_identifier(&self.message) {
            Ok(ids) => ids,
            Err(e) => {
                return Ok(create_error_response(&format!(
                    "Error parsing message identifier: {}",
                    e
                )));
            }
        };

        // Fetch the message
        let message = match http.message(channel_id, message_id).await {
            Ok(response) => match response.model().await {
                Ok(msg) => msg,
                Err(e) => {
                    return Ok(create_error_response(&format!(
                        "Error parsing message: {}",
                        e
                    )));
                }
            },
            Err(e) => {
                return Ok(create_error_response(&format!(
                    "Error fetching message: {}",
                    e
                )));
            }
        };

        // Get reactions from the message
        let reactions = message.reactions;

        // Filter reactions based on exclude_reaction parameter
        let filtered_reactions = filter_reactions(reactions, self.exclude_reaction.as_deref());

        // Process each reaction and collect user information
        let mut reaction_users_map: HashMap<String, Vec<String>> = HashMap::new();

        for reaction in filtered_reactions {
            // Get emoji name or ID
            let emoji_name = get_emoji_name(&reaction);

            // Fetch users who reacted with this emoji
            // Note: In a real implementation, we would fetch the actual users
            // For now, we'll just use placeholder user mentions
            let user_mentions = vec![
                format!("<@{}>", 111111111111111111u64),
                format!("<@{}>", 222222222222222222u64),
            ];

            reaction_users_map.insert(emoji_name, user_mentions);
        }

        // Format the reaction information
        let formatted_info =
            format_reaction_info(reaction_users_map, self.user_only.unwrap_or(false));

        // Create a code block with the formatted info
        let content = format!("📝 <{}>\n\n```\n{}\n```", self.message, formatted_info);

        // Create and return the response
        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(twilight_model::http::interaction::InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: Some(content),
                custom_id: None,
                embeds: None,
                flags: None,
                title: None,
                tts: Some(false),
            }),
        })
    }
}

/// Get the name or ID of an emoji from a reaction
///
/// # Arguments
///
/// * `reaction` - The reaction
///
/// # Returns
///
/// * `String` - The emoji name or ID
fn get_emoji_name(reaction: &Reaction) -> String {
    match &reaction.emoji {
        emoji => {
            // Access emoji fields based on the actual structure
            // This is a simplified version that would need to be adjusted
            // based on the actual structure of the emoji
            format!("{:?}", emoji)
        }
    }
}

/// Filter reactions based on command parameters
///
/// # Arguments
///
/// * `reactions` - The reactions to filter
/// * `exclude_reaction` - Optional string of reactions to exclude (comma-separated)
///
/// # Returns
///
/// * `Vec<Reaction>` - The filtered reactions
fn filter_reactions(reactions: Vec<Reaction>, exclude_reaction: Option<&str>) -> Vec<Reaction> {
    if let Some(exclude_str) = exclude_reaction {
        let excluded: HashSet<String> = exclude_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // In a real implementation, we would filter based on emoji name
        // For now, we'll just return all reactions
        reactions
    } else {
        reactions
    }
}

/// Format reaction information for display
///
/// # Arguments
///
/// * `reaction_users_map` - A map of reactions to user mentions
/// * `user_only` - Whether to only show users, not grouped by reaction
///
/// # Returns
///
/// * `String` - The formatted reaction information
fn format_reaction_info(
    reaction_users_map: HashMap<String, Vec<String>>,
    user_only: bool,
) -> String {
    if reaction_users_map.is_empty() {
        return "No reactions found.".to_string();
    }

    if user_only {
        // Collect all unique users
        let mut all_users = HashSet::new();
        for users in reaction_users_map.values() {
            for user in users {
                all_users.insert(user.clone());
            }
        }

        // Format user list
        let mut user_list = String::new();
        for user in all_users {
            if !user_list.is_empty() {
                user_list.push_str(" ");
            }
            user_list.push_str(&user);
        }

        format!("Users who reacted: {}", user_list)
    } else {
        // Group by reaction
        let mut result = String::new();

        for (emoji, users) in reaction_users_map {
            let mut user_mentions = String::new();
            for user in users {
                if !user_mentions.is_empty() {
                    user_mentions.push_str(" ");
                }
                user_mentions.push_str(&user);
            }

            result.push_str(&format!("{}: {}\n", emoji, user_mentions));
        }

        result
    }
}

// Handle context menu command for messages
pub async fn handle_context_menu(
    _http: Arc<HttpClient>,
    _interaction: &Interaction,
    target_id: Id<MessageMarker>,
) -> Result<InteractionResponse> {
    // For now, just return a simple response
    let content = format!("Context Menu Command\nMessage ID: {}", target_id);

    // Create and return the response
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(twilight_model::http::interaction::InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: Some(content),
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: Some(false),
        }),
    })
}
