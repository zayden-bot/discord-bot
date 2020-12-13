module.exports = (client) => {
    const channelId = "787504014223867904" // Welcome Channel

    client.on("guildMemberAdd", (member) => {

        if (Math.floor(Math.random() * 2) == 0) {
            member.roles.add(config.team1Id).catch(console.error);
            var team = "Team1"
        } else {
            member.roles.add(config.team2Id).catch(console.error);
            var team = "Team2"
        }
            
        const message = `Welcome <@${member.id}> to the server! You have been added to the ${team}`
        const channel = member.guild.channels.cache.get(channelId)

        channel.send(message)
    })
}