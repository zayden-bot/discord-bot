module.exports = {
    commands: "randomise",
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        message.guild.members.cache.filter(m => !m.user.bot).forEach(member => {
            if (!(member.roles.cache.get(config.team1Id)) && !(member.roles.cache.get(config.team2Id))) {
                if (Math.floor(Math.random() * 2) == 0) {
                    member.roles.add(config.team1Id).catch(console.error);
                } else {
                    member.roles.add(config.team2Id).catch(console.error);
                }
            }
        })
        message.reply("Teams have been randomised")
    },
    permissions: ["ADMINISTRATOR"],
}