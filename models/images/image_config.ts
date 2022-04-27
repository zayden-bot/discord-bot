import mongoose from "mongoose";

export interface IImageConfig {
    category: string,
    global: string[],
    users: Map<string, string[]>

    save(): Promise<IImageConfig>;
}

const ImageConfigSchema = new mongoose.Schema({
    category: String,
    global: [],
    users: {type: Map, of: [String]}
})

export const Image_config = mongoose.model("ImageConfigs", ImageConfigSchema)