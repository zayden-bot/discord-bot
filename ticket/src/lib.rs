use serenity::all::{ButtonStyle, ChannelId, Context, CreateButton, CreateMessage, Mention};

pub mod components;
pub mod error;
pub mod message_command;
pub mod modal;
pub mod ready;
pub mod slash_commands;
pub mod support_guild_manager;
pub mod ticket_manager;

pub use components::TicketComponent;
pub use error::Error;
use error::Result;
pub use message_command::SupportMessageCommand;
pub use modal::TicketModal;
pub use slash_commands::{SupportCommand, TicketCommand};
pub use support_guild_manager::TicketGuildManager;
pub use ticket_manager::TicketManager;

pub struct Support;
pub struct Ticket;

pub fn thread_name(thread_id: i32, author_name: &str, content: &str) -> String {
    format!("{} - {} - {}", thread_id, author_name, content)
        .chars()
        .take(100)
        .collect()
}

pub async fn send_support_message(
    ctx: &Context,
    thread_id: ChannelId,
    mentions: &[Mention],
    mut messages: Vec<CreateMessage>,
) -> Result<()> {
    let mentions = mentions
        .iter()
        .map(|mention| mention.to_string())
        .collect::<String>();

    let button = CreateButton::new("support_close")
        .label("Close")
        .style(ButtonStyle::Primary);

    if messages.len() == 1 {
        thread_id
            .send_message(
                ctx,
                messages.pop().unwrap().content(mentions).button(button),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let last_idx = messages.len() - 1;

    for (i, message) in messages.into_iter().enumerate() {
        if i == 0 {
            thread_id
                .send_message(ctx, message.content(&mentions))
                .await
                .unwrap();

            continue;
        }

        if i == last_idx {
            thread_id
                .send_message(ctx, message.button(button.clone()))
                .await
                .unwrap();

            continue;
        }

        thread_id.send_message(ctx, message).await.unwrap();
    }

    Ok(())
}

fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + chars.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
