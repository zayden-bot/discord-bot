import Discord from "discord.js";

module.exports = {
    commands: ["serverLockdown"],
    callback: async (message: Discord.Message) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        await message.reply("Command disabled")
        return;

        // let [channelManager, roleManager] = await Promise.all([guild.channels.fetch(), guild.roles.fetch()])
        // const channels = channelManager.values()
        // const roles = roleManager.values()
        //
        // let lockdownList = []
        //
        // for (const role of roles) {
        //     lockdownList.push({id: role.id, deny: ["SEND_MESSAGES"]})
        // }
        // lockdownList.push({id: roleManager.cache.get("787004533963358279"), allow: ["SEND_MESSAGES"]})
        //
        // for (const channel of channels) {
        //     channel.updateOverwrite(lockdownList, "Server Lockdown")
        // }
    },
    permissions: ["ADMINISTRATOR"],
}