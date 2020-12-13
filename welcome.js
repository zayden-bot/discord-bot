module.exports = (client) => {
    const channelId = "787504014223867904" // Welcome Channel
    const team1Id = "787495858245599252" // Team 1
    const team2Id = "787495874687008768"

    client.on("guildMemberAdd", (member) => {

        if (Math.floor(Math.random() * 2) == 0) {
            member.roles.add(team1Id).catch(console.error);
            var team = "Team1"
        } else {
            member.roles.add(team2Id).catch(console.error);
            var team = "Team2"
        }
            
        const message = `Welcome <@${member.id}> to the server! You have been added to the ${team}`
        const channel = member.guild.channels.cache.get(channelId)

        channel.send(message)
    })
}