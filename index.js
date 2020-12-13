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
    })

//// working on - oscar
    command(client, "rd", message => {
        console.log(typeof message.guild.members.fetch())
        const users = typeof message.guild.members.fetch()

        // message.guild.members.fetch().then(
        //     forEach(member => {
        //         if (Math.floor(Math.random() * 2) == 0) {
        //             member.roles.add(config.team1Id).catch(console.error);
        //             var team = "Team1"
        //             message.channel.send(`${member.username} is now ${team}`)
        //         } else {
        //             member.roles.add(config.team2Id).catch(console.error);
        //             var team = "Team2"
        //             message.channel.send(`${member.username} is now ${team}`)
        //         }
        //     })
        // )
    })
});

client.login(config.token)
