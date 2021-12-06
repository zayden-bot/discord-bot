// module.exports = (client) => {
//     client.on("messageCreate", message => {
//         const { guild, member } = message
//         // if (user is not in databse) { add user to database }
//     })
// }

// const addXP = async (guildId, userId, xpToAdd) => {
//     try {
//         /* Find guildId, userId
//         Update: guildId, userId Increase xp => xpToAdd */
//     } finally {
//         // Close database
//     }
// }

module.exports = {
    commands: "workinprogress", // rank
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        message.reply("Work in progress")
    },
    permissions: ["ADMINISTRATOR"],
}