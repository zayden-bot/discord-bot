const Discord = require("discord.js")
const client = new Discord.Client()
const path = require("path")
const fs = require("fs")

const loadCommands = require("./commands/load-commands")
const config = require("./config.json")
const welcome = require("./welcome");
const sql = require("./sql");
const pingSteve = require("./pingSteve")
const updateRules = require("./rules")
const yesMaster = require("./yesMaster")
const questionMe = require("./questionMe")

client.on("ready", async () => {
    console.log("Zayden is Running");

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

    // welcome(client)

    pingSteve(client)

    // activityTracker(client)

    updateRules(client, "747430712617074718") // Rules Channel ID

    yesMaster(client);

    questionMe(client)
});

client.login(config.token)
