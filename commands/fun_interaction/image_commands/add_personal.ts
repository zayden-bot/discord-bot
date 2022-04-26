import Discord from "discord.js"

module.exports = {
    commands: ["add_personal"],
    expectedArgs: "<category>, <member>, <image_link>",
    minArgs: 3,
    maxArgs: 3,
    callback: async (message: Discord.Message, args: string[]) => {
        if (message.author.id != "211486447369322506") {
            return;
        }

        const imageConfig = require("../../../configs/image_config.json")
        const category = args[0]
        const memberId = require("../../../common").parseId(args[1])
        const imageLink = args[2]

        if (!(category in imageConfig)) {
            await message.reply("Category not found in image config.")
            return;
        }

        if (memberId in imageConfig[category]) {
            imageConfig[category][memberId].push(imageLink)
        } else {
            imageConfig[category][memberId] = [imageLink]
        }

        require("../../../init").updateImages()

        const fs = require("fs")
        fs.writeFile(`./configs/image_config.json`, JSON.stringify(imageConfig, null, 4), (error: any) => {
            if (error) {
                return console.log(error);
            }
        });

        await message.reply(`Successfully added \"${imageLink}\" to ${category}[\"${memberId}\"]`)
    },
}