import mongoose from "mongoose";

export interface IImageConfig {
    category: string,
    global: string[],
    users: Record<string, string[]>

    save(): Promise<IImageConfig>;
}

const ImageConfigSchema = new mongoose.Schema<IImageConfig>({
    category: String,
    global: [],
    users: {type: Map, of: [String]}
})

export const ImageConfig = mongoose.model("ImageConfigs", ImageConfigSchema)
