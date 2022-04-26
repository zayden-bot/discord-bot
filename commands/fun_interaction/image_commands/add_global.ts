import Discord from "discord.js"

module.exports = {
    commands: ["add_global"],
    expectedArgs: "<category>, <image_link>",
    minArgs: 2,
    maxArgs: 2,
    callback: async (message: Discord.Message, args: string[]) => {
        if (message.author.id != "211486447369322506") {
            return;
        }

        const imageConfig = require("../../../configs/image_config.json")
        const category = args[0]
        const image_link = args[1]

        if (!(category in imageConfig)) {
            await message.reply("Category not found in image config.")
            return;
        }

        imageConfig[category].global.push(image_link)

        const fs = require("fs")
        fs.writeFile(`./configs/image_config.json`, JSON.stringify(imageConfig, null, 4), (error: any) => {
            if (error) {
                return console.log(error);
            }
        });
    },
}