import mongoose from "mongoose";

const ImageConfigSchema = new mongoose.Schema({
    category: String,
    global: [],
    users: {type: Map, of: [String]}
})

export const ImageConfig = mongoose.model("ImageConfigs", ImageConfigSchema)