import Discord from "discord.js";
import dotenv from "dotenv";
import fs from "fs";
import path from "path";
import { Server, servers } from "./server";


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


// Init
client.on("ready", () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`Zayden is Running, version: ${botConfig.version}`);

    if (client.user) {
        client.user.setPresence({ activities: [{ name: "College Kings" }], status: "online" })
    }

    const loadCommands = require("./commands/load_commands");
    loadCommands(client)

    // Load server configs
    const serverConfigFiles = fs.readdirSync(path.join(__dirname, "server_configs"))
    for (const filename of serverConfigFiles) {
        const serverConfig = require(path.join(__dirname, "server_configs", filename))
        const guildId = path.parse(filename).name

        const server = new Server(guildId)
        server.reactionRoles = serverConfig.reactionRoles
        server.disabledCommands = serverConfig.disabledCommands
        server.roles = serverConfig.roles
        server.channels = serverConfig.channels
        server.idNumber = serverConfig.idNumber
        server.gameVersions = serverConfig.gameVersions
        server.serverRules = serverConfig.serverRules
        server.serverGuidelines = serverConfig.serverGuidelines
        server.hidden = serverConfig.hidden
        server.moderation = serverConfig.moderation
        server.supportAnswers = serverConfig.supportAnswers

        servers[guildId] = server
    }

    client.guilds.cache.each(guild => {
        if (!(guild.id in servers)) {
            const server = new Server(guild.id)
            servers[guild.id] = server

            fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }

        // Cache reaction messages
        for (let reactionRole of servers[guild.id].reactionRoles) {
            const channel = client.channels.cache.get(reactionRole.channelId) as Discord.TextChannel
            if (!channel) { break; }
            reactionRole.channelId = channel.id;

            channel.messages.fetch(reactionRole.messageId)
                .then((msg: Discord.Message) => { reactionRole.messageId = msg.id })

            guild.roles.fetch(reactionRole.roleId)
                .then(role => { if (role) { reactionRole.roleId = role.id; } })
        }
    })


    // init.updateImages();

    const blacklist = require("./blacklist")
    blacklist.init()

    // const moderation = require("./moderationFunctions")
    // moderation.init()

    // Self Updating
    const update_guidelines = require("./self_updating/updateGuidelines");
    // update_guidelines(client, "879894434538459157")

    const customRoles = require("./self_updating/customRoles")
    customRoles(client, "805765564504473641")

    const updateInfomation = require("./self_updating/updateInfomation")
    updateInfomation(client, "830927865784565800")

    const updateRules = require("./self_updating/updateRules")
    updateRules(client, "747430712617074718")
});


client.on("guildCreate", async guild => {
    const server = new Server(guild.id)

    fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
})


client.on("guildDelete", async guild => {
    fs.unlink(`./server_configs/${guild.id}.json`, (error) => {
        if (error) { console.log(error) }
    })
})


client.on("messageCreate", message => {
    const yesMaster = require("./special_commands/yesMaster")
    yesMaster(message)

    const questionMe = require("./special_commands/questionMe")
    questionMe(message)

    const autoSupport = require("./special_commands/autoSupport")
    autoSupport(message)
})


client.on("messageReactionAdd", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = servers[guild.id];

    for (const reactionRole of server.reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== "907635513341644861") {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) { break; }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) { break; }

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
            if (!member) { break; }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) { break; }

            member.roles.remove(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})

// Events
client.on("guildMemberUpdate", (oldMember, newMember) => {
    const server_config = require("./server_configs/745662812335898806.json");

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
            .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }) as string)
            .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
            .addField("User", `<@${newMember.id}>`, true)
            .addField("Amount", `$${patreonRoles[newRole.id]}`, true)
            .setTimestamp();

        const channel = client.channels.cache.get(server_config.channels.patreonChannel)
        if (channel && channel.isText()) {
            channel.send({ embeds: [embed] });
        }

    }
})

client.on("disconnect", () => {
    console.log("Bot shutting down.")
})

client.on("error", error => {
    console.log(`Error Encountered`);
})

client.login(process.env.TOKEN)

process.on("uncaughtException", (error) => {
    fs.writeFileSync("crash.txt", `Uncaught Exception: ${error.message}`);
    process.exit(1);
})

process.on("unhandledRejection", (reason: Error, promise) => {
    fs.writeFileSync("crash.txt", `Unhandled rejection at ${promise}, reason: ${reason.message}`);
    process.exit(1);
})