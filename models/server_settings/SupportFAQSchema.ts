import mongoose from "mongoose";

export interface ISupportFAQ {
    supportId: string,
    answer: string,
}

export const SupportFAQSchema = new mongoose.Schema<ISupportFAQ>({
    supportId: String,
    answer: String,
})
