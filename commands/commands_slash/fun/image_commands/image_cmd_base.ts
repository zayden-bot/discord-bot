import {IImageConfig, ImageConfig} from "../../../../models/image-config";

export async function getImage(memberId: string, imageArray: string) {
    const imageConfig: IImageConfig | null = await ImageConfig.findOne({category: imageArray})

    if (!imageConfig) return imageConfig

    if (!imageConfig.users) {
        imageConfig.users = {}
    }

    let images = imageConfig.global;
    if (memberId in imageConfig.users) {
        images = imageConfig.users[memberId]
    }

    const imgId = Math.floor(Math.random() * images.length)

    return images[imgId]
}
