// const Discord = require("discord.js")
const fs = require("fs")

module.exports = {
    commands: ["support", "helpme"],
    expectedArgs: "<text>",
    callback: async (message, arguments, text) => {
        const config = require(`../../serverConfigs/${message.guild.id}.json`)
        const idNumber = config.idNumber.toLocaleString('en', {minimumIntegerDigits: 4, useGrouping: false})

        // Send error if in wrong channel
        if (!config.supportChannels.includes(message.channel.id)) {

            // Join support channels together
            let supportChannelsString = ""
            config.supportChannels.forEach(function (channel, index) {
                if (index === config.supportChannels.length - 1) {
                    supportChannelsString += `<#${channel}>`
                } else {
                    supportChannelsString += `<#${channel}> or `
                }
            })

            message.reply(`Wrong channel, please try again in ${supportChannelsString}`)
            return
        }

        // Create channel thread and send mentions
        const thread = await message.channel.threads.create({
            name: idNumber,
            autoArchiveDuration: 1440,
        })
        if (text) {
            thread.send("<@&913374071239102504>")
            thread.send(`${message.author} wrote:\n> ${text}`)
        } else { 
            thread.send(`<@&913374071239102504> ${message.author}`)
        }
        
        // Update json file.
        config.idNumber += 1
        fs.writeFile(`./serverConfigs/${message.guild.id}.json`, JSON.stringify(config, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
    },
}
