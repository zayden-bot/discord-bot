import mongoose from "mongoose";

export interface IUserConfig {
    id: string
    stars: {
        numberOfStars: number,
        givenStars: number,
        receivedStars: number,
    },
    tradingCards: {
        unopenedCardPacks: number,
        openedCardPacks: number
    }

    save(): Promise<IUserConfig>;
}

export async function getUserConfig(id: string) {
    return await UserConfig.findOne({id: id}).exec() || new UserConfig({id: id})
}

const UserConfigSchema = new mongoose.Schema<IUserConfig>({
    id: String,
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

export const UserConfig = mongoose.model("UserConfig", UserConfigSchema)
