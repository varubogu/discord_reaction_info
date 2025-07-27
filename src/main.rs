use dotenv::dotenv;
use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_interactions::command::{CommandInputData, CommandModel};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::{InteractionData, InteractionType};
use twilight_model::id::Id;

mod commands;
mod utils;

use utils::create_error_response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    // Use intents to receive guild message events and reactions.
    let mut shard = Shard::new(
        ShardId::ONE,
        token.clone(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGE_REACTIONS,
    );

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token.clone()));

    // Register application commands
    let application_id = http.current_user_application().await?.model().await?.id;

    tracing::info!("Registering application commands...");
    let commands = commands::register_commands();

    http.interaction(application_id)
        .set_global_commands(&commands)
        .await?;

    tracing::info!("Application commands registered successfully!");

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = DefaultInMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Process each event as they come in.
    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id).content("Pong!").await?;
        }
        Event::InteractionCreate(interaction) => {
            if interaction.kind == InteractionType::ApplicationCommand {
                if let Some(InteractionData::ApplicationCommand(command_data)) = &interaction.data {
                    // Create a simple response acknowledging the command
                    let content = format!(
                        "Received command: {}\n\nThis is a placeholder response. The actual implementation would process the command.",
                        command_data.name
                    );

                    let response = twilight_model::http::interaction::InteractionResponse {
                        kind: twilight_model::http::interaction::InteractionResponseType::ChannelMessageWithSource,
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
                    };

                    // Send the response
                    if let Err(e) = http
                        .interaction(interaction.application_id)
                        .create_response(interaction.id, &interaction.token, &response)
                        .await
                    {
                        tracing::error!("Error sending interaction response: {:?}", e);
                    }
                } else {
                    tracing::warn!("Received unknown interaction data type");
                }
            } else {
                tracing::warn!("Received unknown interaction type: {:?}", interaction.kind);
            }
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
