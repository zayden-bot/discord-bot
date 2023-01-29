import Discord from "discord.js"
import {getConnection} from "../../../../servers";
import {IImageSchema} from "../../../../models/global/IImageSchema";

const imageCategories = [
    {name: "GoodMorning", value: "GoodMorningImages"},
    {name: "GoodNight", value: "GoodNightImages"},
    {name: "Hug", value: "HugImages"}
]

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("add_global_image")
        .setDescription("Add an image link to the global index")
        .addStringOption(option =>
            option.setName("category")
                .setDescription("The image category")
                .setRequired(true)
                .addChoices(...imageCategories))
        .addStringOption(option =>
            option.setName("image_link")
                .setDescription("The image link")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const category = interaction.options.getString("category", true)
        const imageLink = interaction.options.getString("image_link", true)

        const conn = getConnection("Global")
        const imageModel = await conn.model<IImageSchema>(category)
        if (!imageModel)
            return interaction.reply("Category not found in image database. Please contact @OscarSix")

        await (await imageModel.create({imageUrl: imageLink})).save()

        await interaction.reply(`Successfully added "${imageLink}" to ${category} images`)
    }
}
