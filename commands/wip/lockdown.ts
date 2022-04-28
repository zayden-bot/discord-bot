import Discord from "discord.js";

module.exports = {
    commands: ["lockdown"],
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
        //     channel.updateOverwrite(roles.cache.get(roleId), {SEND_MESSAGES: false}, "Channel Lockdown")
        // }

        // channel.updateOverwrite(roles.cache.get("787004533963358279"), {SEND_MESSAGES: true}, "Channel Lockdown")
        message.channel.send("Channel Lockdown! Please listen to the staff for further information.")
    },
    requiredRoles: ["Admin"],
}

