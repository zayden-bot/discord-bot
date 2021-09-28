module.exports = {
    commands: ["punish"],
    expectedArgs: "<user>",
    minArgs: 1,
    maxArgs: 1,
    permissionError: "You are not a Master! Security Breach calling <@211486447369322506>!",
    callback: (message, arguments, text) => {
        const serverConfig = require(`../../serverConfigs/${message.guild.id}.json`)
        const member = message.mentions.members.first()

        if (!member) {
            message.reply("Please enter a valid member")
            return;
        }

        if (serverConfig.masters.includes(member.id)) {
            message.channel.send(`Sorry Master Oscar, I cannot punish Master ${member.user.username}!`);
            return;
        }
        message.channel.send(`<@${member.id}> bow down to Master ${message.author.username}! Or you will recieve an hour in the pain chamber.`)

    },
    permissions: ["ADMINISTRATOR"],
}