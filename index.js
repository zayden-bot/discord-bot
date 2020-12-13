const Discord = require("discord.js")
const client = new Discord.Client()

const config = require("./config.json")
const command = require("./command"); 
const welcome = require("./welcome");
const sql = require("./sql");
const activityTracker = require("./activityTracker")

client.on("ready", () => {
    console.log("College King's Bot is Running");

    sql.init() // keep it here so it connects to the database

    //                CREATE EXAMPLE
    //  sql.run("CREATE TABLE IF NOT EXISTS test (`val` INT NOT NULL DEFAULT '1')");
    //
    //                SELECT EXAMPLE
    //  sql.each("SELECT * FROM test ORDER BY val DESC", (row) => {
    //      console.log(row.val)
    //  });
    //
    //                CLOSE CONNECTION
    //  sql.end() 
    //
    //                CREATE CONNECTION
    //  sql.init()
    //
    
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

    command(client, "suggest", message => {
        const suggestion = message.content.replace("!suggest", "")

        const embed = new Discord.MessageEmbed()
            .setTitle(`From: ${message.author.username}`)
            .setDescription(suggestion)

        let channel = message.guild.channels.cache.get(config.suggestionChannel)
        channel.send(embed).then(function(message) {
            message.react("👍")
            message.react("👎")
        })
        
    })

    command(client, "rules", message => {
        const embed = new Discord.MessageEmbed()
            .setTitle(`From: ${message.author.username}`)
            .setDescription(suggestion)
        
            channel.send(embed)
    })
});

client.login(config.token)
