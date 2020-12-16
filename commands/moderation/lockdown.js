const memberRoles = [
    "745663432560345218", // King
    "745663409932206112", // President
    "745663394543304704", // Senior
    "745663375496708127", // Junior
    "745663351756947656", // Sophomore
    "745663316776714370", // Freshman
    "768568151343497257", // Server Booster
    "787447252783202326", // Ultra Fan
    "787447090728796191", // Mega Fan
    "787446715057831976", // Super Fan
    "787445900992577556", // Big Fan
    "787445571539304510", // Active Fan
    "787443819024220210", // New Fan
    "745662812335898806" // @everyone
]

module.exports = {
    commands: ["lockdown"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const channel = message.channel;
        const roles = message.guild.roles;

        for (roleId of memberRoles) {
            channel.updateOverwrite(roles.cache.get(roleId), {SEND_MESSAGES: false}, "Channel Lockdown")
        }

        channel.updateOverwrite(roles.cache.get("787004533963358279"), {SEND_MESSAGES: true}, "Channel Lockdown")

    },
    requiredRoles: ["Security"],
}

