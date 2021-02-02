if (message.content.startsWith("!enter")) {
    const member = guild.members.cache.find(member => member.id === "ID")
    guild.roles.cache.find((role) => role.name === "PP Gang")
    member.roles.add(role) 
}