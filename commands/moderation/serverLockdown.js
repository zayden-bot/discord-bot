module.exports = {
    commands: ["serverLockdown"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const channels = message.guild.channels.cache.array();
        const roles = message.guild.roles;

        let lockdownList = []

        for (roleId of memberRoles) {
            lockdownList.push({ id: roles.cache.get(roleId), deny: ["SEND_MESSAGES"], })
        }
        lockdownList.push({ id: roles.cache.get("787004533963358279"), allow: ["SEND_MESSAGES"], })

        for (channel of channels) {
            channel.updateOverwrite(lockdownList, "Server Lockdown")
        }

        lockdownList = []

    },
    permissions: ["ADMINISTRATOR"],
}