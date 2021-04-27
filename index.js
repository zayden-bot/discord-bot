const Discord = require("discord.js")
const fs = require("fs")

const client = new Discord.Client({ partials: ['MESSAGE', 'CHANNEL', 'REACTION'] })

const loadCommands = require("./commands/load-commands");
const botConfig = require("./configs/botConfig.json");
const music = require("./musicFunctions")

let servers = {};

client.on("ready", async () => {
    console.log(`Zayden is Running, version: ${botConfig.version}`);

    client.user.setPresence({
        activity: {
            name: "College Kings",
            type: 0,
        },
    })

    const guilds = client.guilds.cache.map(guild => guild.id)
    for (guild of guilds) {
        servers[guild] = {}
        servers[guild].moderation = {}
        servers[guild].reactionRoles = {}
        servers[guild].queue = new music.Queue(guild)
    }
    module.exports = { servers: servers }
    
    // Connect to database
    const sql = require("./sql");
    sql.init(); 

    loadCommands(client)

    const moderation = require("./moderationFunctions")
    moderation.init()

    const reactionRoles = require("./reactionRoleFuncions")
    reactionRoles.init();
    reactionRoles.addNormalReactionRole(client);
    reactionRoles.removeNormalReactionRole(client);

    const updateClubs = require("./selfUpdating/updateClubs")
    updateClubs.customClubs(client, "805765564504473641")
    updateClubs.pledgeRoles(client, "805765564504473641")

    const updateInfomation = require("./selfUpdating/updateInfomation")
    updateInfomation(client, "830927865784565800")

    const updateRules = require("./selfUpdating/updateRules")
    updateRules(client, "747430712617074718")

});

client.on("message", message => {
    const yesMaster = require("./specialCommands/yesMaster")
    yesMaster(message)

    const questionMe = require("./specialCommands/questionMe")
    questionMe(message)
})

client.on("guildCreate", guild => {
    const defaultConfig = {
        "disabledCommands": [],
        "staffRoles": [],
        "suggestionChannel": "",
        "logsChannel": "",
        "patreonChannel": "",
        "questionChannel": "",
        "patreonUpdate": "",
        "steamUpdate": "",
        "serverRules": {},
        "hiddenRules": {},
        "masters": []
    }

    fs.writeFile(`./serverConfigs/${guild.id}.json`, JSON.stringify(defaultConfig, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
})

client.on("guildDelete", async guild => {
    fs.unlink(`./serverConfigs/${guild.id}.json`)
})

client.on("disconnect", () => {
    console.log("Bot shutting down.")
})

client.on("error", error => {
    console.log(`Error Encountered`);
})

client.login(botConfig.token)