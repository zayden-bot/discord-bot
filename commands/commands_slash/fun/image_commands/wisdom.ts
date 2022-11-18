// module.exports = {
//     data: new Discord.SlashCommandBuilder()
//         .setName("wisdom")
//         .setDescription("Send a hug message")
//         .addUserOption(option =>
//             option.setName("member")
//                 .setDescription("Member to give a hug too")),
//
//     async execute(interaction: Discord.ChatInputCommandInteraction) {
//         if (!(interaction.member instanceof Discord.GuildMember)) {
//             return;
//         }
//
//         const member = interaction.options.getMember("member") || interaction.member
//         if (!(member instanceof Discord.GuildMember)) {
//             return interaction.reply("Unknown member mentioned");
//         }
//
//         const image = await getImage(interaction.member.id, "hug")
//         if (!image) {
//             return interaction.reply("No \"hug\" image found")
//         }
//
//         const embed = new Discord.EmbedBuilder()
//             .setTitle(`Sending hugs to ${member.displayName}`)
//             .setImage(image)
//
//         interaction.reply({embeds: [embed]}).then()
//     }
// }


// module.exports = {
//     commands: ["wisdomoftheday", "wisdom", "w"],
//     callback: async (message: Discord.Message) => {
//         await message.reply("Work in progress")
//         return
//
//         const imageConfig: IImageConfig | null = await ImageConfig.findOne<IImageConfig>({category: "wisdom"}).exec()
//
//         let images = imageConfig!.global;
//         if (message.author.id in imageConfig!.users) {
//             images = imageConfig!.users[message.author.id]
//         }
//
//         // Returns 0 - 365
//         const now = new Date();
//         const start = new Date(now.getFullYear(), 0, 0);
//         const oneDay = 1000 * 60 * 60 * 24;
//         const imageIndex = Math.floor((now.valueOf() - start.valueOf()) / oneDay)
//
//         const image = images[imageIndex]
//
//         // Check if index is within bounds of the global images
//         if (image) {
//             const embed = new Discord.EmbedBuilder()
//                 .setTitle("Today's Wisdom")
//                 .setImage(image)
//
//             message.channel.send({embeds: [embed]})
//         } else {
//         }
//
//     },
// }
