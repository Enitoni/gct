use std::{env, sync::Arc};

use crate::{
    audio::AudioSystem,
    discord::{util, Context, Error},
};
use poise::{
    serenity::client,
    serenity_prelude::{ChannelId, GatewayIntents, GuildId},
};
use songbird::{SerenityInit, Songbird};

// This is GCT's Discords server
const HOME_GUILD_ID: u64 = 671811819597201421;

// The channel to join for streaming
const VOICE_CHANNEL_ID: u64 = 671859933876191265;

pub struct Bot {
    audio: Arc<AudioSystem>,
    voice: Arc<Songbird>,
}

impl Bot {
    pub async fn run(audio: Arc<AudioSystem>) {
        let token = env::var("GCT_DISCORD_TOKEN").expect("GCT_DISCORD_TOKEN was not specified.");
        let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

        let commands = {
            let mut list = vec![register()];

            list.extend(util::commands().into_iter());
            list
        };

        let bot = Bot {
            voice: Songbird::serenity(),
            audio,
        };

        let voice = bot.voice.clone();

        let framework = poise::Framework::build()
            .options(poise::FrameworkOptions {
                commands,
                ..Default::default()
            })
            .token(token)
            .intents(intents)
            .client_settings(move |client| client.register_songbird_with(voice))
            .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(bot) }));

        framework.run().await.unwrap();
    }
}

/// Update or delete application commands
#[poise::command(slash_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
