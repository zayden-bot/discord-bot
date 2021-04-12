const Discord = require("discord.js")
const client = new Discord.Client({ partials: ['MESSAGE', 'CHANNEL', 'REACTION'] })
const path = require("path")
const fs = require("fs")

const loadCommands = require("./commands/load-commands");
const config = require("./Configs/botConfig.json");
const sql = require("./sql");
const updateRules = require("./selfUpdating/updateRules");
const updateInfomation = require("./selfUpdating/updateInfomation");
const updateClubs = require("./selfUpdating/updateClubs");
const yesMaster = require("./yesMaster");
const questionMe = require("./questionMe");
const blacklist = require("./blacklist");
const reactionRoles = require("./reactionRoles");


// Temp event fix
const guildMemberUpdateLog = require("./events/logs/guildMemberUpdate.js");

client.on("ready", async () => {
    console.log("Zayden is Running");

    client.user.setPresence({
        activity: {
            name: "College Kings",
            type: 0,
        },
    })

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

    updateRules(client, "747430712617074718") // Rules Channel ID
    updateInfomation(client, "830927865784565800") // information Channel ID
    updateClubs.customClubs(client, "805765564504473641") // Clubs Channel ID
    updateClubs.pledgeRoles(client, "805765564504473641")
    
    yesMaster(client);

    questionMe(client);

    blacklist.init();

    reactionRoles.init()
    reactionRoles.addNormalReactionRole(client)
    reactionRoles.removeNormalReactionRole(client)

});

client.login(config.token)
