import Discord from "discord.js";

module.exports = {
    commands: ["unlock"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        // const channel = message.channel;
        // const roles = message.guild.roles;

        // for (roleId of memberRoles) {
        //     channel.updateOverwrite(roles.cache.get(roleId), {SEND_MESSAGES: true}, "Channel Unlocked")
        // }
        message.channel.send("Channel Unlocked! Thank you for your patience")
    },
    requiredRoles: ["Admin"],
}
