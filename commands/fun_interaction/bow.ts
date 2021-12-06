import Discord from "discord.js"

module.exports = {
    commands: ["bow"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message, args: string[], text: string) => {
        message.channel.send(`*rapidly bowing* sorry it won't happen again.`)
        message.channel.send("https://media1.giphy.com/media/pGM4yi3ql6SeA/giphy.gif?cid=ecf05e4754mdzlrqlmag4l0s24dooz0jqw8sd0poa5lkjk7c&rid=giphy.gif&ct=g")

    },
}