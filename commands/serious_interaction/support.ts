import Discord from "discord.js"

module.exports = {
    commands: ["support", "helpme"],
    expectedArgs: "<text>",
    callback: async (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild || message.channel.type !== "GUILD_TEXT") { return; }

        const config = require(`../../server_configs/${message.guild.id}.json`)
        const idNumber = config.idNumber.toLocaleString('en', {minimumIntegerDigits: 4, useGrouping: false})

        // Send error if in wrong channel
        if (!config.supportChannels.includes(message.channel.id)) {

            // Join support channels together
            let supportChannelsString = ""
            config.supportChannels.forEach((channel: string, index: number) => {
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

        const fs = require("fs")
        fs.writeFile(`./server_configs/${message.guild.id}.json`, JSON.stringify(config, null, 4), (err: any) => {
            if (err) { return console.log(err); }
        });
    },
}
