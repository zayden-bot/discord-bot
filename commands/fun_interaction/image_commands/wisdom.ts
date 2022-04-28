import Discord from "discord.js";
import {IImageConfig, ImageConfig} from "../../../models/image-config";

module.exports = {
    commands: ["wisdomoftheday", "wisdom", "w"],
    callback: async (message: Discord.Message) => {
        const imageConfig: IImageConfig = await ImageConfig.findOne({category: "wisdom"}).exec()

        let images = imageConfig.global;
        if (message.author.id in imageConfig.users) {
            images = imageConfig.users[message.author.id]
        }

        // Returns 0 - 365
        const now = new Date();
        const start = new Date(now.getFullYear(), 0, 0);
        const oneDay = 1000 * 60 * 60 * 24;
        const imageIndex = Math.floor((now.valueOf() - start.valueOf()) / oneDay)

        const image = images[imageIndex]

        // Check if index is within bounds of the global images
        if (image) {
            const embed = new Discord.MessageEmbed()
                .setTitle("Today's Wisdom")
                .setImage(image)

            message.channel.send({embeds: [embed]})
        } else {
        }

    },
}