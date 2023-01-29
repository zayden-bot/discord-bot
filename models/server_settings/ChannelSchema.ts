import mongoose from "mongoose";

export interface IChannel {
    category: string
    name: string,
    type: string,
    id: string
}

export const ChannelSchema = new mongoose.Schema<IChannel>({
    category: String,
    name: String,
    type: String,
    id: String
})
