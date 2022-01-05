import Discord from "discord.js";
import { servers } from "../server";

module.exports = async function (message: Discord.Message) {
    const guild = message.guild
    if (!guild
        || !message.content
        || !message.member
        || message.channel.type !== "GUILD_TEXT"
        || message.author.id == "787490197943091211") { return; }

    const server = servers[message.guild.id]
    if (!server.channels.supportChannels ||
        !server.channels.supportChannels.includes(message.channel.id) ||
        message.member.roles.cache.has(server.roles.moderationRole) ||
        message.member.roles.cache.has(server.roles.supportRole)) {
        return;
    }
    const idNumber = server.idNumber.toLocaleString('en', { minimumIntegerDigits: 4, useGrouping: false })

    // Create channel thread and send mentions
    let threadName = `${idNumber} - ${message.content}`
    if (threadName.length > 100) { threadName = threadName.substring(0, 100) }

    const thread = await message.channel.threads.create({
        name: threadName,
        autoArchiveDuration: 1440,
    })

    if (message.content.length > 2000) {
        thread.send(`<@&913374071239102504> ${message.author} Content length over 2000 characters please resend message.`)
    }
    else {
        thread.send(`<@&913374071239102504> ${message.author} wrote:`)
        thread.send({
            content: message.content,
            embeds: message.embeds,
            files: [...message.attachments.values()]
        })
    }

    // Update json file.
    server.idNumber += 1

    const fs = require("fs")
    fs.writeFile(`./server_configs/${message.guild.id}.json`, JSON.stringify(server, null, 4), (err: any) => {
        if (err) { return console.log(err); }
    });

    message.delete()
    message.channel.bulkDelete(1)
}