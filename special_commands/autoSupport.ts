import Discord from "discord.js"

module.exports = async function(message: Discord.Message) {
    const guild = message.guild
    if (!guild || message.channel.type !== "GUILD_TEXT" || message.author.id == "787490197943091211" || !message.member) { return; }

    const config = require(`../server_configs/${guild.id}.json`)
    if (!config.channels.supportChannels || !config.channels.supportChannels.includes(message.channel.id) || message.member.roles.cache.has(config.moderationRole)) { return; }
    const idNumber = config.idNumber.toLocaleString('en', {minimumIntegerDigits: 4, useGrouping: false})

    // Create channel thread and send mentions
    let threadName = `${idNumber} - ${message.content}`
    if (threadName.length > 100) {  threadName = threadName.substring(0, 100) }

    const thread = await message.channel.threads.create({
        name: threadName,
        autoArchiveDuration: 1440,
    })

    thread.send("<@&913374071239102504>")
    thread.send(`${message.author} wrote:\n> ${message.content}`)

    // Update json file.
    config.idNumber += 1

    const fs = require("fs")
    fs.writeFile(`./server_configs/${message.guild.id}.json`, JSON.stringify(config, null, 4), (err: any) => {
        if (err) { return console.log(err); }
    });

    message.delete()
    message.channel.bulkDelete(1)
}