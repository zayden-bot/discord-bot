import mongoose from "mongoose";
import {getConnection} from "../servers";

export interface IUserConfig {
    id: string
    infractions: number
    stars: {
        numberOfStars: number,
        givenStars: number,
        receivedStars: number,
    },
    tradingCards: {
        unopenedCardPacks: number,
        openedCardPacks: number
    }
}

export async function getUserConfig(userId: string) {
    const conn = getConnection("Zayden")
    let user = await conn.model<IUserConfig>("UserConfig").findOne({id: userId})
    if (!user)
        user = await conn.model<IUserConfig>("UserConfig").create({id: userId})

    return user
}

export const UserConfigSchema = new mongoose.Schema<IUserConfig>({
    id: String,
    infractions: {type: Number, default: 0},
    stars: {
        numberOfStars: {type: Number, default: 0},
        givenStars: {type: Number, default: 0},
        receivedStars: {type: Number, default: 0},
    },
    tradingCards: {
        unopenedCardPacks: {type: Number, default: 0},
        openedCardPacks: {type: Number, default: 0}
    }
})
