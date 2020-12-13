module.exports = (client) => {
    channelId = "787504014223867904" // Welcome Channel
    team1Id = "787495858245599252" // Team 1
    team2Id = "787495874687008768"

    client.on("guildMemberAdd", member => {
        console.log(member)

        const team1 = message.guild.roles.get(team1Id)
        const team2 = message.guild.roles.get(team2Id)

        if (Math.floor(Math.random() * 2) == 0) {
            member.addRole(team1).catch(console.error);
            const team = "Team1"
        } else {
            member.addRole(team2).catch(console.error);
            const team = "Team2"
        }
            
        const message = `Welcome <@${member.id}> to the server! You have been added to the ${team}`
        const channel = member.guild.channels.cache.get(channelId)

        channel.send(message)
    })
}


// let team1 = message.guild.roles.get(config.team1);
// let team2 = message.guild.roles.get(config.team2);

client.on('guildMemberAdd', member => {

});
