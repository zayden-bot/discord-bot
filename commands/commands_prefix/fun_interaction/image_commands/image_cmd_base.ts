import {IImageConfig, ImageConfig} from "../../../../models/image-config";
import Discord from "discord.js";

export async function getImage(author: Discord.User, imageArray: string) {
    const imageConfig: IImageConfig | null = await ImageConfig.findOne<IImageConfig>({category: imageArray}).exec()

    if (!imageConfig!.users) {
        imageConfig!.users = {}
    }

    let images = imageConfig!.global;
    if (author.id in imageConfig!.users) {
        images = imageConfig!.users[author.id]
    }

    const imgId = Math.floor(Math.random() * images.length)

    return images[imgId]
}
