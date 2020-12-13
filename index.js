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
            .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
            .setDescription("**1.** Do not do anything which breaks the Discord ToS or Community Guidelines.\n\n**2.** Do not harass, bully or cause drama with other members within the discord.\n\n**3.** Racism, Sexism, Homophobia or any other offensive subject matters are strictly forbidden.\n**3a.** Talking about Politics, Religion and other sensitive subjects are also forbidden\n\n**4.** Do not spam. This includes Images, Repeatedly Asking Questions or spamming emojis.\n\n**5.** No Advertising. This includes but not limited to Discord Servers or other websites that aren't related to College Kings. Offical websites to other games are allowed in <#772516507041005618>\n\n**6.** Do not threaten to DDoS or dox someone, it is also prohibited to discuss these topics or share information regarding either topic (As well as discuss information gained via a dox).\n\n**7.** Do not post any NSFW pictures outside of an NSFW marked channel. Gore, Loli, Shota and other Extreme NSFW content is prohibited.\n\n**8.** This is an English-only server.\n\n**9.** Stay on-topic in the respective channels\n\n**10.** Respect our staff team, their decisions are final.\n\n\n**If you do not agree/abide with these rules, you will get kicked or banned from the server. Here at College Kings you are to follow our Discord's Community Guidelines.**")
            .setColor("ff0000")
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        message.reply(embed)
    })

    command(client, "patreon", message => {
        const embed = new Discord.MessageEmbed()
            .setTitle("Pledge to College Kings")
            .setURL(`https://www.patreon.com/collegekings`)
            .setDescription("**Interested In Getting Early Updates, Patron-only behind the scenes/post... and more?\n\nCheck it all out here!**\nhttps://www.patreon.com/collegekings")
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")
            
        message.reply(embed)
    })
});

client.login(config.token)
