pub mod rinfo;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::CommandType;

#[allow(deprecated)]
pub fn register_commands() -> Vec<twilight_model::application::command::Command> {
    vec![
        rinfo::RinfoCommand::create_command().into(),
        twilight_model::application::command::Command {
            application_id: None,
            default_member_permissions: None,
            description: "Get reaction information for a message".to_string(),
            description_localizations: None,
            // TODO: This field is deprecated. Should use contexts instead when the correct import for CommandContext is known.
            dm_permission: None,
            guild_id: None,
            id: None,
            integration_types: None,
            contexts: None,
            name: "Reaction Info".to_string(),
            name_localizations: None,
            nsfw: None,
            options: vec![],
            kind: CommandType::Message,
            version: twilight_model::id::Id::new(1),
        },
    ]
}
