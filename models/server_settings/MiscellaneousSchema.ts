import mongoose from "mongoose";

export interface IMiscellaneous {
    enabledModules: string[]
    moderationRoles: string[];
    supportRoles: string[]
    supportThreadId: number
}

export const MiscellaneousSchema = new mongoose.Schema<IMiscellaneous>({
    enabledModules: [String],
    supportThreadId: {type: Number, default: 0},
    moderationRoles: [String],
    supportRoles: [String]
})
