import Discord from "discord.js";
import dotenv from "dotenv";
import {Zayden} from "./client"
import {loadMessageCommands, loadSlashCommands} from "./commands/load_commands";
import {saveAndCloseDbConnections} from "./mongoDb";
import deployCommands from "./deploy_commands";
import {IReactionRole} from "./models/server_settings/ReactionRoleSchema";
import {IChannel} from "./models/server_settings/ChannelSchema";
import {getConnection} from "./servers";


if (process.env.NODE_ENV == "development") {
    dotenv.config({path: "./.env.local"})
} else {
    dotenv.config()
}

export const client = new Zayden({
    intents: [
        Discord.GatewayIntentBits.Guilds,
        Discord.GatewayIntentBits.GuildMessages,
        Discord.GatewayIntentBits.GuildMembers,
        Discord.GatewayIntentBits.GuildMessageReactions,
        Discord.GatewayIntentBits.MessageContent,
    ],
    partials: [
        Discord.Partials.Message,
        Discord.Partials.Channel,
        Discord.Partials.Reaction,
    ]
})

client.on("ready", async () => {
    console.log(`${client.constructor.name} is Running, version: ${client.version.join('.')}`);

    client.user?.setPresence({activities: client.activities, status: "online"})

    // Initialize Servers
    await require("./servers").init(client)

    loadMessageCommands(client)
    loadSlashCommands(client)

    if (process.env.NODE_ENV == "development") {
        await deployCommands(client)
    }

    // Self Updating
    const customRoles = require("./self_updating/customRoles")
    await customRoles(client, "805765564504473641")

    const updateInformation = require("./self_updating/updateInfomation")
    await updateInformation(client, "830927865784565800")
    //
    // const updateRules = require("./self_updating/updateRules")
    // await updateRules(client, "747430712617074718")
});

client.on(Discord.Events.MessageCreate, message => {
    const client = message.client as Zayden

    Promise.all(Array.from(client.messageCommands.values()).map(command => command.callback(message))).then()
})

client.on(Discord.Events.MessageReactionAdd, async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const conn = getConnection(guild.id)
    const reactionRoles = await conn.model<IReactionRole>("ReactionRoles").find()

    for (const reactionRole of reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== "907635513341644861") {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) {
                break;
            }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) {
                break;
            }

            member.roles.add(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})

client.on(Discord.Events.MessageReactionRemove, async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const conn = getConnection(guild.id)
    const reactionRoles = await conn.model<IReactionRole>("ReactionRoles").find()

    for (const reactionRole of reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== "907635513341644861") {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) {
                break;
            }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) {
                break;
            }

            member.roles.remove(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})

client.on(Discord.Events.GuildMemberUpdate, async (oldMember, newMember) => {
    const guild = newMember.guild

    const conn = getConnection(guild.id)
    const channels = await conn.model<IChannel>("Channels").findOne({category: "patreonChannel"})
    if (!channels) return;

    const patreonRoles: Record<string, number> = {
        '745663316776714370': 1, // Freshman
        '745663351756947656': 5, // Sophomore
        '745663375496708127': 10, // Junior
        '745663394543304704': 20, // Senior
        '745663409932206112': 50, // President
        '745663432560345218': 100 // King
    }

    // Get new added role
    const newRole = newMember.roles.cache
        .filter(role => !oldMember.roles.cache.has(role.id))
        .first()

    // Is new role a patreon role
    if (typeof (newRole) != "undefined" && newRole.id in patreonRoles) {
        const embed = new Discord.EmbedBuilder()
            .setTitle("New Patron")
            .setColor(`${newRole.hexColor}`)
            .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
            .addFields([
                {name: "User", value: `<@${newMember.id}>`, inline: true},
                {name: "Amount", value: `$${patreonRoles[newRole.id]}`, inline: true}
            ])
            .setTimestamp();

        const serverIconURL = guild.iconURL()
        if (serverIconURL) {
            embed.setFooter({text: guild.name, iconURL: serverIconURL})
        }

        const channel = client.channels.cache.get(channels.id)
        if (channel && channel.type == Discord.ChannelType.GuildText) {
            channel.send({embeds: [embed]});
        }
    }
})

client.on(Discord.Events.InteractionCreate, async interaction => {
    if (!interaction.isChatInputCommand()) return;

    const client = interaction.client as Zayden
    const command = client.slashCommands?.get(interaction.commandName);

    if (!command) {
        console.error(`No command matching ${interaction.commandName} was found.`);
        return;
    }

    try {
        await command.execute(interaction)
    } catch (error) {
        console.error(error);
        await interaction.reply({
            content: "There was an error while executing this command!",
            ephemeral: true
        })
    }
})

client.login(process.env.TOKEN).then();

if (process.env.NODE_ENV != "development") {
    process.on("uncaughtException", async (error) => {
        await saveAndCloseDbConnections()

        console.error(error)
    })

    process.on("unhandledRejection", async (reason, promise) => {
        await saveAndCloseDbConnections()

        console.error(`Unhandled Rejection at: ${promise}  reason: ${reason}`)
    })
}
