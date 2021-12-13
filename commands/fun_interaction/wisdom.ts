import Discord from "discord.js";

module.exports = {
    commands: ["wisdomoftheday", "wisdom", "w"],
    callback: (message: Discord.Message, args: string[], text: string) => {
        const images = require("../../configs/image_config.json").huggingImgs.Global

        // Returns 0 - 365
        const now = new Date();
        const start = new Date(now.getFullYear(), 0, 0);
        const oneDay = 1000 * 60 * 60 * 24;
        const imageIndex = Math.floor((now.valueOf() - start.valueOf()) / oneDay)

        // Check if index is within bounds of the images
        if (imageIndex < images.length) { 
            const embed = new Discord.MessageEmbed()
                .setTitle("Today's Wisdom")
                .setImage(images[imageIndex])

            message.channel.send({embeds: [embed]})
        } else { 
            message.reply("There is no wisdom for today. #BlameJany")
        }

    },
}