const Discord = require("discord.js")

const { prefix } = require("../configs/botConfig.json");
const blacklist = require("../blacklist.js");
const developerUsers = ["211486447369322506"]
const staffRole = "787004533963358279"

const validatePermissions = (permissions) => {
    const validPermissions = [
        "CREATE_INSTANT_INVITE",
        "KICK_MEMBERS",
        "BAN_MEMBERS" ,
        "ADMINISTRATOR",
        "MANAGE_CHANNELS",
        "MANAGE_GUILD",
        "ADD_REACTIONS",
        "VIEW_AUDIT_LOG",
        "PRIORITY_SPEAKER",
        "STREAM",
        "VIEW_CHANNEL",
        "SEND_MESSAGES",
        "SEND_TTS_MESSAGES",
        "MANAGE_MESSAGES",
        "EMBED_LINKS",
        "ATTACH_FILES",
        "READ_MESSAGE_HISTORY",
        "MENTION_EVERYONE",
        "USE_EXTERNAL_EMOJIS",
        "VIEW_GUILD_INSIGHTS",
        "CONNECT",
        "SPEAK",
        "MUTE_MEMBERS",
        "DEAFEN_MEMBERS",
        "MOVE_MEMBERS",
        "USE_VAD",
        "CHANGE_NICKNAME",
        "MANAGE_NICKNAMES",
        "MANAGE_ROLES",
        "MANAGE_WEBHOOKS",
        "MANAGE_EMOJIS",
    ]

    for (const permission of permissions) {
        if (!validPermissions.includes(permission)) {
            throw new Error(`Unkown permission node "${permission}"`)
        }
    }
}

let recentlyRan = []

module.exports = (client, commandOptions) => {
    let {
        commands,
        expectedArgs = "",
        permissionError = "You do not have permission to run this command",
        minArgs = 0,
        maxArgs = null,
        cooldown = -1,
        permissions = [],
        requiredRoles = [],
        disabled = false,
        callback
    } = commandOptions

    if (typeof commands === "string") {
        commands = [commands];
    }

    if (disabled) { return }

    console.log(`Registering command "${commands[0]}"`);

    if (permissions.length) {
        if (typeof permissions == "string") {
            permissions = [permissions]
        }

        validatePermissions(permissions)
    }

    if (requiredRoles.length) {
        if (typeof requiredRoles == "string") {
            requiredRoles = [requiredRoles]
        }
    }

    client.on("message", message => {
        const { member, content, guild, channel } = message
        const serverConfig = require(`../serverConfigs/${guild.id}.json`)
        const testServer = "745662812335898806"

        for (const alias of commands) {
            if (content.split(" ")[0].toLowerCase() == `${prefix}${alias.toLowerCase()}`) {

                // Check if the command is globally disabled
                if (disabled) { return }

                // Check if the command is enabled in that server
                if (!(commands[0].toLowerCase() in serverConfig.enabledCommands) && guild.id != testServer) { return }

                // Check if the user has the correct permissions to run the command
                for (const permission of permissions) {
                    if (!member.hasPermission(permission) && !developerUsers.includes(member.id)) {
                        message.reply(permissionError)
                        return
                    }
                }

                // Check if the user has the required roles to run the command
                for (const requiredRole of requiredRoles) {
                    const role = guild.roles.cache.find(role => role.name === requiredRole)

                    if (!role || !member.roles.cache.has(role.id) && !developerUsers.includes(member.id)) {
                        message.reply(permissionError)
                        return
                    }
                }

                // Check if the user is blacklisted
                if (blacklist.isBlacklisted(member.user.id) && !developerUsers.includes(member.id)) {
                    return
                }

                // Check if the command is on cooldown
                let cooldownString = `${guild.id}-${member.id}-${commands[0]}`
                if (cooldown > 0 && recentlyRan.includes(cooldownString) && !member.roles.cache.has(staffRole)) {
                    message.reply("You cannot use that command so soon, please wait")
                    return
                }

                // Create the arguments variable
                const arguments = content.split(/[ ]+/)
                arguments.shift()

                // Check if the user inputed the correct number of arguments
                if (arguments.length < minArgs || ( maxArgs !== null && arguments.length > maxArgs )) {
                    const embed = new Discord.MessageEmbed()
                    .setColor("#ff0000")
                    .setDescription(`Invalid command usage, try using it like:\n\`${prefix}${alias} ${expectedArgs}\``)
                    message.channel.send(embed)
                    return
                }

                // Add command to recentlyRan if command has cooldown
                if (cooldown > 0) {
                    recentlyRan.push(cooldownString)

                    setTimeout(() => {
                        recentlyRan = recentlyRan.filter((string) => {
                            return string !== cooldownString
                        })
                    }, 1000 * cooldown);
                }

                console.log(`Running ${prefix}${alias}`)
                callback(message, arguments, arguments.join(" "))

                return
            }
        }
    }).setMaxListeners(0)
}
