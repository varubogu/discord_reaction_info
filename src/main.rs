use std::{env, error::Error, sync::Arc};
use dotenv::dotenv;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    // Use intents to only receive guild message events.
    let mut shard = Shard::new(
        ShardId::ONE,
        token.clone(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));

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
            http.create_message(msg.channel_id)
                .content("Pong!")
                .await?;
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}