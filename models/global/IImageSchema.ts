import mongoose from "mongoose";

export interface IImageSchema {
    imageUrl: string
    users: string[],
}

export const FuckImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

export const GoodMorningImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

export const GoodNightImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

export const HugImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

export const KissImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

export const SlapImageSchema = new mongoose.Schema<IImageSchema>({
    imageUrl: String,
    users: [String]
})

