const Discord = require("discord.js")
const client = new Discord.Client()

const config = require("./config.json")
const command = require("./command"); 
const welcome = require("./welcome");
const activityTracker = require("./activityTracker")

client.on("ready", () => {
    console.log("College King's Bot is Running");
    
    welcome(client)

    activityTracker(client)

    command(client, "ping", (message) => {
        message.channel.send("Pong!")
    })

    command(client, "serverinfo", (message) => {
        client.guilds.cache.forEach((guild) => {
            message.channel.send(`**${guild.memberCount}** total members`)
        })
    })

    command(client, ["cc", "clearchannel", "purgeall"], message => {
        if (message.member.hasPermission("ADMINISTRATOR")) {
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
        message.reply(`Status Changed to: Playing ${content}`)
    })

    command(client, "randomise", message => {
        if (message.member.hasPermission("ADMINISTRATOR")) {
            message.guild.members.cache.filter(m => !m.user.bot).forEach(member => {
                if (!(member.roles.cache.get(config.team1Id)) && !(member.roles.cache.get(config.team2Id))) {
                    if (Math.floor(Math.random() * 2) == 0) {
                        member.roles.add(config.team1Id).catch(console.error);
                    } else {
                        member.roles.add(config.team2Id).catch(console.error);
                    }
                }
            })
            message.reply("Teams have been randomised")
        }
    })
});

client.login(config.token)
