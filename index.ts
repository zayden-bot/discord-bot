import Discord from "discord.js"
import dotenv from "dotenv"
import { Server, servers } from "./server";
import fs from "fs"
import path from "path"
dotenv.config()

// const init = require("./init")



const client = new Discord.Client({
    intents: [
        Discord.Intents.FLAGS.GUILDS,
        Discord.Intents.FLAGS.GUILD_MESSAGES,
        Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS
    ],
    partials: ['MESSAGE', 'CHANNEL', 'REACTION']
})


// Init
client.on("ready", () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`Zayden is Running, version: ${botConfig.version}`);

    const loadCommands = require("./commands/load_commands");
    loadCommands(client)

    // Load server configs
    const serverConfigFiles = fs.readdirSync(path.join(__dirname, "server_configs"))
    for (const filename of serverConfigFiles) {
        const file = require(path.join(__dirname, "server_configs", filename))
        const guildId = path.parse(filename).name
        const server = new Server(guildId, file)
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
            reactionRole.channel = client.channels.cache.get(reactionRole.channel.id) as Discord.TextChannel;

            reactionRole.channel.messages.fetch(reactionRole.message.id)
                .then((msg: Discord.Message) => { reactionRole.message = msg })

            guild.roles.fetch(reactionRole.role.id)
                .then(role => { if (role) reactionRole.role = role; })
        }
    })

    
    // init.updateImages();

    // const moderation = require("./moderationFunctions")
    // moderation.init()

    // const reactionRoles = require("./reactionRoleFuncions")
    // reactionRoles.init();
    // reactionRoles.addNormalReactionRole(client);
    // reactionRoles.removeNormalReactionRole(client);

    // Self Updating
    // const update_guidelines = require("./selfUpdating/updateGuidelines");
    // update_guidelines(client, "879894434538459157")

    // const customRoles = require("./selfUpdating/custom_roles")
    // customRoles(client, "805765564504473641")

    // const updateInfomation = require("./selfUpdating/updateInfomation")
    // updateInfomation(client, "830927865784565800")

    // const updateRules = require("./selfUpdating/updateRules")
    // updateRules(client, "747430712617074718")

});


client.on("guildCreate", async guild => {
    const server = new Server(guild.id)

    fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
})


client.on("guildDelete", async guild => {
    fs.unlink(`./server_configs/${guild.id}.json`, (error) => {
        if (error) {console.log(error)}
    })
})


// client.on("messageCreate", message => {
//     const yesMaster = require("./specialCommands/yesMaster")
//     yesMaster(message)

//     const questionMe = require("./specialCommands/questionMe")
//     questionMe(message)
// })


// client.on("messageReactionAdd", (reaction, user) => {
//     if (!reaction.message.guild) return;

//     const guild = reaction.message.guild
//     const server = servers[guild.id];

//     for (const reactionRole of server.reactionRoles) {
//         if (reaction.message == reactionRole.message && reaction.emoji.name == reactionRole.emoji && user.id !== "907635513341644861") {
//             const member = guild.members.cache.find(member => member.id == user.id)
//             if (member) {  member.roles.add(reactionRole.role) }
//             break
//         }
//     }
// })


// client.on("messageReactionRemove", (reaction, user) => {
//     if (!reaction.message.guild) return;

//     const guild = reaction.message.guild
//     const server = servers[guild.id];

//     for (const reactionRole of server.reactionRoles) {
//         if (reaction.message == reactionRole.message && reaction.emoji.name == reactionRole.emoji && user.id !== "907635513341644861") {
//             const member = guild.members.cache.find(member => member.id == user.id)
//             if (member) {  member.roles.remove(reactionRole.role) }
//             break
//         }
//     }
// })

// Events
// client.on("guildMemberUpdate", (oldMember, newMember) =>{
//     const { patreonChannel } = require("./server_configs/745662812335898806.json");

//     const patreonRoles: Record<string, number> = {
//         '745663316776714370': 1, // Freshman
//         '745663351756947656': 5, // Sophomore
//         '745663375496708127': 10, // Junior
//         '745663394543304704': 20, // Senior
//         '745663409932206112': 50, // President
//         '745663432560345218': 100 // King
//     }

//     // Get new added role
//     const newRole = newMember.roles.cache
//         .filter(role => !oldMember.roles.cache.has(role.id))
//         .first()

//     // Is new role a patreon role
//     if (typeof(newRole) != "undefined" && newRole.id in patreonRoles) {
//         const embed = new Discord.MessageEmbed()
//         .setTitle("New Patron")
//         .setColor(`${newRole.hexColor}`)
//         .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }) as string)
//         .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
//         .addField("User", `<@${newMember.id}>`, true)
//         .addField("Amount", `$${patreonRoles[newRole.id]}`, true)
//         .setTimestamp();

//         const channel = client.channels.cache.get(patreonChannel) as Discord.TextChannel
//         channel.send({embeds: [embed]});
//     }
// })

client.on("disconnect", () => {
    console.log("Bot shutting down.")
})

client.on("error", error => {
    console.log(`Error Encountered`);
})

client.login(process.env.TOKEN)