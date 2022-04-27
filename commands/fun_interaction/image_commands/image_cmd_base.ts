import {IImageConfig, ImageConfig} from "../../../models/images/image-config";
import Discord from "discord.js";

export async function getImage(author: Discord.User, imageArray: string) {
    const imageConfig: IImageConfig = await ImageConfig.findOne({category: imageArray}).exec()
    
    let images = imageConfig.global;
    if (author.id in imageConfig.users) {
        images = imageConfig.users[author.id]
    }

    const imgId = Math.floor(Math.random() * images.length)

    return images[imgId]
}