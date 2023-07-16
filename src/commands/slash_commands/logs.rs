use chrono::{Months, Utc};
use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::models::Infraction;
use crate::sqlx_lib::get_user_infractions;
use crate::utils::{respond_with_embed, respond_with_message};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let user = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::User(user, _member)) => user,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
    };

    let member = match guild_id.member(&ctx, &user.id).await {
        Ok(member) => member.to_owned(),
        Err(_) => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
    };

    let filter = match interaction.data.options.get(1) {
        Some(filter) => match filter.resolved.as_ref() {
            Some(CommandDataOptionValue::String(filter)) => filter,
            _ => return respond_with_message(ctx, interaction, "Invalid filter option").await,
        },
        None => "recent",
    };

    let mut infractions = match get_user_infractions(member.user.id.0 as i64).await {
        Ok(user_infractions) => user_infractions,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting user config").await,
    };

    if filter != "all" {
        let six_months_age = Utc::now().checked_sub_months(Months::new(6)).unwrap().naive_utc();

        infractions = infractions.into_iter().filter(|infraction| infraction.created_at >= six_months_age).collect::<Vec<Infraction>>();
    }

    respond_with_embed(ctx, interaction, |e| {
        e.title(format!("Logs for {}", member.display_name()));

        for infraction in infractions {
            let field_body = format!("**Type:** {}\n", infraction.infraction_type)
                + &format!("**User:** ({}) {}\n", infraction.user_id, infraction.username)
                + &format!("**Moderator:** ({}) {}\n", infraction.moderator_id, infraction.moderator_username)
                + &format!("**Reason:** {}", infraction.reason);

            e.field(format!("Case #{}", infraction.id), field_body, false);
        }

        e
    }).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("logs")
        .description("Get logs for a user")
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
        .create_option(|option| {
            option.name("user")
                .description("The user to get logs for")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option.name("filter")
                .description("The number of logs to get")
                .kind(CommandOptionType::String)
                .add_string_choice("Recent (default)", "recent")
                .add_string_choice("All", "all")
        })
}