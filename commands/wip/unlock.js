module.exports = {
    commands: ["unlock"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const channel = message.channel;
        const roles = message.guild.roles;

        // for (roleId of memberRoles) {
        //     channel.updateOverwrite(roles.cache.get(roleId), {SEND_MESSAGES: true}, "Channel Unlocked")
        // }
        message.channel.send("Channel Unlocked! Thank you for your patience")
    },
    requiredRoles: ["Admin"],
}
