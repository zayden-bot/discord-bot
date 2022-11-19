// module.exports = (client) => {
//     client.on("messageCreate", message => {
//         const { guild, member } = message
//         // if (user is not in database) { add user to database }
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

import Discord from "discord.js";

module.exports = {
    commands: "workinprogress", // rank
    callback: async (message: Discord.Message) => {
        await message.reply("Work in progress")
    },
    permissions: ["ADMINISTRATOR"],
}