import mongoose from "mongoose";

interface IBotBan {
    caseNumber: number,
    userId: string,
    logType: string,
    moderatorId: string,
    reason: string
}

export interface IBotConfig {
    botBan: IBotBan[]

    save(): Promise<IBotConfig>;
}

const BotConfigSchema = new mongoose.Schema<IBotConfig>({
    botBan: [{
        caseNumber: Number,
        userId: String,
        logType: String,
        moderatorId: String,
        reason: String
    }]

})

export const BotConfig = mongoose.model("BotConfig", BotConfigSchema)
