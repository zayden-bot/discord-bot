const config = require("./config.json")

module.exports = (client) => {
    const channelId = "747426208341426196" // Welcome Channel
    var team = "None"

    client.on("guildMemberAdd", (member) => {
        // if (Math.floor(Math.random() * 2) == 0) {
        //     member.roles.add(config.team1Id).catch(console.error);
        //     var team = "Team1"
        // } else {
        //     member.roles.add(config.team2Id).catch(console.error);
        //     var team = "Team2"
        // }
        if (team == "None") {
            var message = `Welcome <@${member.id}> to the server!`
        } else {
            var message = `Welcome <@${member.id}> to the server! You have been added to ${team}`
        }
        
        const channel = member.guild.channels.cache.get(channelId)
        if (!channel) { return }
        channel.send(message)
        console.log(`Welcoming ${member}`)
    })
}