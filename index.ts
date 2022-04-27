import Discord from "discord.js";
import dotenv from "dotenv";
import fs from "fs";
import {createServer, servers} from "./servers";

dotenv.config()

export const client = new Discord.Client({
    intents: [
        Discord.Intents.FLAGS.GUILDS,
        Discord.Intents.FLAGS.GUILD_MEMBERS,
        Discord.Intents.FLAGS.GUILD_MESSAGES,
        Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS
    ],
    partials: ['MESSAGE', 'CHANNEL', 'REACTION']
})


// Initialize database
require("./sql").init()
const inits = require("./init")
inits.updateImages()

// Init
client.on("ready", async () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`Zayden is Running, version: ${botConfig.version}`);

    if (client.user) {
        client.user.setPresence({activities: [{name: "College Kings"}], status: "online"})
    }

    // Initialize Servers
    await require("./servers").init(client)

    const loadCommands = require("./commands/load_commands");
    loadCommands(client)

    const blacklist = require("./blacklist")
    blacklist.init()

    // const moderation = require("./moderationFunctions")
    // moderation.init()

    // Self Updating

    const customRoles = require("./self_updating/customRoles")
    await customRoles(client, "805765564504473641")

    const updateInformation = require("./self_updating/updateInfomation")
    await updateInformation(client, "830927865784565800")

    const updateRules = require("./self_updating/updateRules")
    await updateRules(client, "747430712617074718")
});


client.on("guildCreate", async guild => {
    await createServer(guild)
})


client.on("guildDelete", async guild => {
    fs.unlink(`./server_configs/${guild.id}.json`, (error) => {
        if (error) {
            console.log(error)
        }
    })
})


client.on("messageCreate", message => {
    const yesMaster = require("./special_commands/yesMaster")
    yesMaster(message)

    const questionMe = require("./special_commands/questionMe")
    questionMe(message)

    const autoSupport = require("./special_commands/autoSupport")
    autoSupport(message).then()
})


client.on("messageReactionAdd", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = servers[guild.id];

    for (const reactionRole of server.reactionRoles) {
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


client.on("messageReactionRemove", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = servers[guild.id];

    for (const reactionRole of server.reactionRoles) {
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

// Events
client.on("guildMemberUpdate", (oldMember, newMember) => {
    const server_config = require(`./server_configs/${newMember.guild.id}.json`);
    const guild = newMember.guild

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
        const embed = new Discord.MessageEmbed()
            .setTitle("New Patron")
            .setColor(`${newRole.hexColor}`)
            .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
            .addField("User", `<@${newMember.id}>`, true)
            .addField("Amount", `$${patreonRoles[newRole.id]}`, true)
            .setTimestamp();

        const serverIconURL = guild.iconURL({dynamic: true})
        if (serverIconURL) {
            embed.setFooter({text: guild.name, iconURL: serverIconURL})
        }

        const channel = client.channels.cache.get(server_config.channels.patreonChannel)
        if (channel && channel.isText()) {
            channel.send({embeds: [embed]});
        }

    }
})

client.login(process.env.TOKEN).then();

process.on("uncaughtException", (error) => {
//     const fs = require("fs")
//         if (member_config != null) {
//             fs.writeFileSync(`./user_configs/${member.id}.json`, JSON.stringify(member_config, null, 4));
//         }
//         if (author_config != null) {
//             fs.writeFileSync(`./user_configs/${author?.id}.json`, JSON.stringify(author_config, null, 4));
//         }
//         if (server_config != null) {
//             fs.writeFileSync(`./server_configs/${server?.id}.json`, JSON.stringify(server_config, null, 4));
//         }
//     },
//
//     parseId: function (id: string): string | undefined {
//     const match = id.match(/\d+/)
//     if (match) { return match[0]; }
// },
//
// updateConfig: function (guild: Discord.Guild, server: Server) {
//     fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), (error: any) => {
//         if (error) { return console.log(error); }
//     });
    console.error(error)
})
//
// process.on("unhandledRejection", (reason: Error, promise) => {
//     console.log(`Unhandled rejection at ${promise}, reason: ${reason.message}`)
//     fs.writeFileSync("crash.txt", `Unhandled rejection at ${promise}, reason: ${reason.message}`);
//     process.exit(1);
// })
// TODO: Update configs on bot exit or crash