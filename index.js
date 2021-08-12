const Discord = require("discord.js")
const fs = require("fs")

const client = new Discord.Client({ partials: ['MESSAGE', 'CHANNEL', 'REACTION'] })

const init = require("./init")
const loadCommands = require("./commands/load-commands");
const botConfig = require("./configs/botConfig.json");

let servers = {};

// Init
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
    }
    module.exports = { servers: servers }
    
    init.updateImages();

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
        "game_version": "",
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

// Events
client.on("guildMemberUpdate", (oldMember, newMember) =>{
    const { patreonChannel } = require("./serverConfigs/745662812335898806.json");
    const patreonRoles = {
        ['745663316776714370'] : 1, // Freshman
        ['745663351756947656'] : 5, // Sophomore
        ['745663375496708127'] : 10, // Junior
        ['745663394543304704'] : 20, // Senior
        ['745663409932206112'] : 50, // President
        ['745663432560345218'] : 100 // King
    }
    // Get new added role
    const newRole = newMember.roles.cache
        .filter(role => !oldMember.roles.cache.has(role.id))
        .first()

    // Is new role a patreon role
    if (typeof(newRole) != "undefined" && newRole.id in patreonRoles) {
        const emebed = new Discord.MessageEmbed()
        .setTitle("New Patron")
        .setColor(`${newRole.hexColor}`)
        .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }))
        .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
        .addField("User", `<@${newMember.id}>`, true)
        .addField("Amount", `$${patreonRoles[newRole.id]}`, true)
        .setTimestamp();

        client.channels.cache.get(patreonChannel).send(emebed);
    }
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