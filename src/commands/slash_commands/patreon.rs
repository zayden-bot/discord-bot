use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;

#[derive(serde::Deserialize, Debug)]
struct PatreonMemberAttributes {
    campaign_lifetime_support_cents: i32,
    email: Option<String>,
    patron_status: Option<String>
}

#[derive(serde::Deserialize, Debug)]
struct PatreonMemberData {
    attributes: PatreonMemberAttributes,
    id: Option<String>,
    r#type: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct PatreonLinks {
    next: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct PatreonPagination {
    total: i32,
}

#[derive(serde::Deserialize, Debug)]
struct PatreonMeta {
    pagination: Option<PatreonPagination>,
}

#[derive(serde::Deserialize, Debug)]
struct PatreonMember {
    data: Vec<PatreonMemberData>,
    patreon_links: Option<PatreonLinks>,
    patreon_meta: Option<PatreonMeta>,
}

fn info(mut response: CreateInteractionResponse) -> CreateInteractionResponse {
    response.interaction_response_data(|message| message.embed(|e| {
        e.title("Pledge to College Kings")
            .url("https://www.patreon.com/collegekings")
            .description("**Interested In Getting Early Updates, Patron-only behind the scenes/post... and more?\n\nCheck it all out here!**\nhttps://www.patreon.com/collegekings")
            .image("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg")
            .thumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png")
            .footer(|f| f.text("https://www.patreon.com/collegekings"))
    }));
    response
}

async fn check<'a>(subcommand: &CommandDataOption, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    response.interaction_response_data(|message| message.content("This command is not yet implemented"));
    return response;

    // let email = match subcommand.options[0].resolved.as_ref() {
    //     Some(CommandDataOptionValue::String(email)) => email,
    //     _ => {
    //         response.interaction_response_data(|message| message.content("Invalid email"));
    //         return response;
    //     }
    // };
    //
    // let res = match reqwest::get(format!("http://81.100.246.35/api/v1/patreon/users/{}", email)).await {
    //     Ok(res) => res,
    //     Err(_) => {
    //         response.interaction_response_data(|message| message.content("Error connecting to College Kings API"));
    //         return response;
    //     },
    // };
    //
    // let patreon_member = res.json::<PatreonMember>().await.unwrap();
    // println!("{:?}", patreon_member);
    //
    // response.interaction_response_data(|message| message.embed(|e| {
    //     e.title("Patreon Status")
    //         .description(format!("Lifetime Support (USD): **{}**\nEmail: {}\nPatreon Status: **{}**", patreon_member.data[0].attributes.campaign_lifetime_support_cents / 100, patreon_member.data[0].attributes.email.as_ref().unwrap(), patreon_member.data[0].attributes.patron_status.as_ref().unwrap()))
    // }));
    // response
}

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let subcommand = &interaction.data.options[0];
    return match subcommand.name.as_str() {
        "info" => info(response),
        "check" => check(subcommand, response).await,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid subcommand"));
            response
        },
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("patreon")
        .description("Patreon information")
        .create_option(|option|
            option.name("info")
                .description("Patreon information")
                .kind(CommandOptionType::SubCommand))
        .create_option(|option|
            option.name("check")
                .description("Check if you're a patron")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option|
                    sub_option.name("email")
                        .description("Your Patreon email")
                        .kind(CommandOptionType::String)
                        .required(true)))
}