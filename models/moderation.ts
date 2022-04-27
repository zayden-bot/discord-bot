import mongoose from "mongoose";

export const ModerationSchema = new mongoose.Schema({
    caseNumber: Number,
    guildId: String,
    userId: String,
    type: String,
    moderatorId: String,
    reason: String
})