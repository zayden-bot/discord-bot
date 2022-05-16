import Discord from "discord.js";
import dotenv from "dotenv";
import {createServer} from "./servers";
import mongoose from "mongoose";
import {Server} from "./models/server";
import {UserConfig} from "./models/user-config";

switch (process.env.NODE_ENV) {
    case "development":
        dotenv.config({path: "./.env.local"})
        break;
    default:
        dotenv.config()
        break;
}

const dbURI = `mongodb+srv://${process.env.DB_USERNAME}:${process.env.DB_PASSWORD}@zayden.wcx6n.mongodb.net/Zayden?retryWrites=true&w=majority`
mongoose.connect(dbURI)

export const client = new Discord.Client({
    intents: [
        Discord.Intents.FLAGS.GUILDS,
        Discord.Intents.FLAGS.GUILD_MEMBERS,
        Discord.Intents.FLAGS.GUILD_MESSAGES,
        Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS
    ],
    partials: ['MESSAGE', 'CHANNEL', 'REACTION']
})

// Init
client.on("ready", async () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`Zayden is Running, version: ${botConfig.version}`);

    if (client.user) {
        client.user.setPresence({activities: [{name: "College Kings"}], status: "online"})
    }

    // Initialize Bot Config
    await require("./bot-config").init()

    // Initialize Servers
    await require("./servers").init(client)

    const loadCommands = require("./commands/load_commands");
    loadCommands(client)

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


client.on("messageReactionAdd", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = await Server.findOne({id: guild.id}).exec()

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

    const server = await Server.findOne({id: guild.id}).exec()

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
client.on("guildMemberUpdate", async (oldMember, newMember) => {
    const guild = newMember.guild
    const server = await Server.findOne({id: guild.id}).exec()

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

        const channel = client.channels.cache.get(server.channels.patreonChannel)
        if (channel && channel.isText()) {
            channel.send({embeds: [embed]});
        }

    }
})

client.login(process.env.TOKEN).then();

async function saveAllDB() {
    let tasks: Promise<any>[] = []

    async function saveServers() {
        for (const server of await Server.find().exec()) {
            tasks.push(server.save())
        }
    }

    async function saveUsers() {
        for (const user of await UserConfig.find().exec()) {
            tasks.push(user.save())
        }
    }

    await Promise.all([saveServers(), saveUsers()])
    return tasks
}

process.on("uncaughtException", async (error) => {
    await Promise.all(await saveAllDB())
    console.error(error)
})

process.on("unhandledRejection", async (reason, promise) => {
    await Promise.all(await saveAllDB())
    console.error(`Unhandled Rejection at: ${promise}  reason: ${reason}`)
})
