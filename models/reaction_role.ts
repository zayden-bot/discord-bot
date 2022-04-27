import mongoose from "mongoose";

export const ReactionRoleSchema = new mongoose.Schema({
    channelId: String,
    messageId: String,
    roleId: String,
    emoji: String
})