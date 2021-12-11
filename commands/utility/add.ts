import Discord from "discord.js"

// Template Command
module.exports = {
    commands: ["add", "addition"],
    expectedArgs: "<num1> <num2> <num3> ...",
    permissionError: "",
    minArgs: 2,
    callback: (message: Discord.Message, args: string[], text: string) => {
        let sum = 0
        for (const arg of args) {
            if (isNaN(Number(arg))) {
                message.reply("Invalid arguments")
                return;
            }
            sum += Number(arg);
        }

        message.reply(`Answer: ${sum}`)
    },
    permissions: ["MANAGE_MESSAGES"],
    requiredRoles: ["Staff"],
}