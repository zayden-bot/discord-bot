const Discord = require("discord.js")
const client = new Discord.Client({ partials: ['MESSAGE', 'CHANNEL', 'REACTION'] })
const path = require("path")
const fs = require("fs")

const loadCommands = require("./commands/load-commands");
const config = require("./config.json");
const sql = require("./sql");
const updateRules = require("./updateRules");
const updateClubs = require("./updateClubs");
const yesMaster = require("./yesMaster");
const questionMe = require("./questionMe");
const blacklist = require("./blacklist");
const dmMatt = require("./dmMatt");
const reactionRoles = require("./reactionRoles");


// Temp event fix
const guildMemberUpdateLog = require("./events/logs/guildMemberUpdate.js");

client.on("ready", async () => {
    console.log("Zayden is Running");

    client.user.setPresence({
        game: {
            name: "College Kings",
            type: "PLAYING",
            url: "https://www.patreon.com/collegekings"
        }
    });

    sql.init(); // keep it here so it connects to the database

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

    loadCommands(client)

    const baseFile = "command-base.js"
    const eventFile = "event-base.js"
    const eventBase = require(`./events/${eventFile}`)

    const readEvents = dir => {
        const files = fs.readdirSync(path.join(__dirname, dir))
        for (const file of files) {
            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readEvents(path.join(dir, file))
            } else if (file !== baseFile) {
                const option = require(path.join(__dirname, dir, file))
                eventBase(client, option)
            }
        }
    }

    readEvents("events")

    client.on("guildMemberUpdate", async (oldMember, newMember) => {
        guildMemberUpdateLog.log(client, oldMember, newMember);
    });

    // welcome(client)

    // activityTracker(client)

    dmMatt(client)

    updateRules(client, "747430712617074718") // Rules Channel ID
    updateClubs(client, "805765564504473641") // Clubs Channel ID

    yesMaster(client);

    questionMe(client);

    blacklist.init();

    reactionRoles.init()
    reactionRoles.addNormalReactionRole(client)
    reactionRoles.removeNormalReactionRole(client)

    // client.on("message", message => {
    //     const media = (message.content.toLowerCase().startsWith("https://") || message.attachments.size > 0)
    //     const channel = (message.channel.id == "747428952577933424" || message.channel.id == "788893358482784266")
        
    //     if (!media && channel && !message.author.bot) {
    //         message.delete({ reason:"Non-nsfw message in NSFW channel" }).then(
    //             message.channel.send("Please only post NSFW media in this channel.").then(msg => { msg.delete({ timeout:10000 }) })
    //         ).catch(() => message.channel.send("Cannot delete message. Missing MANAGE_MESSAGES permission."));
    //     }
    // })

});

client.login(config.token)
