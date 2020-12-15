const path = require("path")
const fs = require("fs")
const Discord = require("discord.js")
const client = new Discord.Client()

const config = require("./config.json")
const welcome = require("./welcome");
const sql = require("./sql");
const pingSteve = require("./pingSteve")

client.on("ready", async () => {
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
    
    const baseFile = "command-base.js"
    const commandBase = require(`./commands/${baseFile}`)

    const readCommands = dir => {
        const files = fs.readdirSync(path.join(__dirname, dir))
        for (const file of files) {
            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
            } else if (file !== baseFile) {
                const option = require(path.join(__dirname, dir, file))
                commandBase(client, option)
            }
        }
    }

    readCommands("commands")

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
});

client.login(config.token)
