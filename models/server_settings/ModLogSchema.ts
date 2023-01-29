import mongoose from "mongoose";

export interface IModLog {
    logId: number,
    userId: string,
    logType: string,
    moderatorId: string,
    reason: string
}

export const ModLogSchema = new mongoose.Schema<IModLog>({
    logId: Number,
    userId: String,
    logType: String,
    moderatorId: String,
    reason: String
})
