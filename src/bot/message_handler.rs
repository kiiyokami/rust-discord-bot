use serenity::all::{Context, EventHandler, Message, CreateWebhook, ExecuteWebhook, CreateButton, CreateActionRow, ButtonStyle};
use crate::services::auto_embedder::auto_embed;
use rand::Rng;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: serenity::all::Interaction) {
        if let serenity::all::Interaction::Component(component) = interaction {
            if component.data.custom_id.starts_with("delete:") {
                let original_user_id = component.data.custom_id.strip_prefix("delete:").unwrap();
                
                let is_admin = component.member.as_ref()
                    .map(|m| m.permissions.unwrap_or_default().administrator())
                    .unwrap_or(false);
                
                let is_original_author = component.user.id.to_string() == original_user_id;
                
                if is_admin || is_original_author {
                    let _ = component.message.delete(&ctx.http).await;
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "pong").await;
        }

        else if msg.content.starts_with("!roll") {
            let max = msg.content.split_whitespace().nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(100);
            let roll = rand::rng().random_range(1..=max);
            let _ = msg.channel_id.say(&ctx.http, format!("🎲 You rolled: {}", roll)).await;
        }
        
        else if msg.content.starts_with("http") {
            if reqwest::get(&msg.content).await.is_ok() {
                if let Some(embedded_url) = auto_embed(&msg.content) {
                    if let Ok(webhook) = msg.channel_id.create_webhook(&ctx.http, CreateWebhook::new(&msg.author.name)).await {
                        let row = CreateActionRow::Buttons(vec![
                            CreateButton::new(format!("delete:{}", msg.author.id)).label("🗑️ Delete").style(ButtonStyle::Danger)
                        ]);
                        
                        let _ = webhook.execute(&ctx.http, false, ExecuteWebhook::new()
                            .content(&embedded_url)
                            .username(&msg.author.name)
                            .avatar_url(&msg.author.avatar_url().unwrap_or_default())
                            .components(vec![row])
                        ).await;
                        
                        let _ = webhook.delete(&ctx.http).await;
                        let _ = msg.delete(&ctx.http).await;
                    }
                }
            }
        }
    }
}
