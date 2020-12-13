const Discord = require("discord.js")
const client = new Discord.Client()

const config = require("./config.json")
const command = require("./command") 
const welcome = require("./welcome")

client.on("ready", () => {
    console.log("College King's Bot is Running")

    welcome(client)

    command(client, "ping", (message) => {
        message.channel.send("Pong!")
    })

    command(client, "serverinfo", (message) => {
        client.guilds.cache.forEach((guild) => {
            message.channel.send(`**${guild.memberCount}** total members`)
        })
    })

    command(client, ["cc", "clearchannel", "purgeall"], message => {
        if (message.member.hasPermission("ADMINISTATOR")) {
            message.channel.messages.fetch().then(results => {
                message.channel.bulkDelete(results)
            })
        }
    })

    command(client, "status", message => {
        const content = message.content.replace("!status", "")
        client.user.setPresence({
            activity: {
                name: content,
                type: 0,
            },
        })
    })
})


client.login(config.token)
