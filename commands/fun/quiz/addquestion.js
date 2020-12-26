const sqlite3 = require("sqlite3").verbose(); // npm install sqlite3
const Discord = require("discord.js")

const config = require("../../../config.json")
const sql = require("../../../sql")

module.exports = {
    commands: ["listquestion", "listq"],
    callback: (message, arguments, text) => {
        let db = new sqlite3.Database(config.dbFile);
        let sqlQuery = `SELECT * FROM quiz`;
        let questions = []

        const embed = new Discord.MessageEmbed()
            .setTitle("Stored Questions")
            .setDescription()
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        sql.each(sqlQuery)

        // db.each(sql, [], (err, rows) => {
        //     if (err) {
        //         console.log(err);
        //     }
        //     questions.push(rows)
        //     // console.log(questions);
        // });
        // // console.log(questions);
        // // close the database connection
        // db.close();
        // console.log(questions);
    },
    // requiredRoles: "Security",
}

