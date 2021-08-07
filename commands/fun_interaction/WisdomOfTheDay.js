const Discord = require("discord.js")
const imgConfig = require("../../configs/imgConfig.json")

function currentdayinyear() {
    const month = new Date().getMonth().toString()
    const day = new Date().getDay().toString()
// somehow return current day of the year
}

module.exports = {
    commands: ["WisdomOfTheDay"],
    expectedArgs: "<date>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let day;
        try { day = Number(arguments[0]) }
        catch (error) { day = currentdayinyear() }

        let arrayId = "Error";
        if (day in imgConfig.wisdomImgs) { arrayId = day }

        const embed = new Discord.MessageEmbed()
            .setImage(imgConfig.wisdomImgs[arrayId][0])

        message.channel.send(embed)
    },
}
