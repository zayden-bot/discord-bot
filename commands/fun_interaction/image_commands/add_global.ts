import Discord from "discord.js"
import {IImageConfig, ImageConfig} from "../../../models/image-config";

module.exports = {
    commands: ["add_global"],
    expectedArgs: "<category>, <image_link>",
    minArgs: 2,
    maxArgs: 2,
    callback: async (message: Discord.Message, server: unknown, args: string[]) => {
        if (message.author.id != "211486447369322506") {
            return;
        }

        const imageLink = args[1]

        const imageConfig: IImageConfig = await ImageConfig.findOne({category: args[0]}).exec()

        if (!imageConfig) {
            await message.reply("Category not found in image config.")
            return;
        }

        const globalImages = new Set(imageConfig.global)
        globalImages.add(imageLink)

        imageConfig.global = [...globalImages]

        await Promise.all([
            imageConfig.save(),
            message.reply(`Successfully added "${imageLink}" to ${imageConfig.category} images`)
        ])
    },
}