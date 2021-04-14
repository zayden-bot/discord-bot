module.exports = {
    commands: ["punish"],
    permissionError: "You are not a Master! Security Breach calling <@211486447369322506>!",
    callback: (message, arguments, text) => {
        if (text == "<@!747423760780623872>") {
            message.channel.send(`Sorry Master Oscar, I cannot punish Master Steve!`)
        } else {
            message.channel.send(`${text} bow down to Master Oscar! Or you will recieve an hour in the pain chamber`)
        }
    },
    permissions: ["ADMINISTRATOR"],
}