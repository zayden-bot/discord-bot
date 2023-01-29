import mongoose from "mongoose";

export interface IQuestion {
    questionId: number,
    text: string,
    userId: string,
    messageId: string | undefined,
    answer: { text: string; userId: string; } | undefined
}

export const QuestionSchema = new mongoose.Schema<IQuestion>({
    questionId: Number,
    text: String,
    userId: String,
    messageId: String,
    answer: {
        text: String,
        userId: String,
    }
})
