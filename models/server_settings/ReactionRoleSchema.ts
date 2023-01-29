import mongoose from "mongoose";

export interface IReactionRole {
    channelId: string,
    messageId: string,
    roleId: string,
    emoji: string
}

export const ReactionRoleSchema = new mongoose.Schema<IReactionRole>({
    channelId: String,
    messageId: String,
    roleId: String,
    emoji: String
})
