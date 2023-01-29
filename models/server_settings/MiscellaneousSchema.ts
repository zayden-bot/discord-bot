import mongoose from "mongoose";

export interface IMiscellaneous {
    supportThreadId: number;
    moderationRoles: string[]
    supportRoles: string[]
}

export const MiscellaneousSchema = new mongoose.Schema<IMiscellaneous>({
    supportThreadId: {type: Number, default: 0},
    moderationRoles: [String],
    supportRoles: [String]
})
