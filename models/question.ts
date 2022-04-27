import mongoose from "mongoose";
import {AnswerSchema} from "./answer";

export const QuestionSchema = new mongoose.Schema({
    text: String,
    userId: String,
    questionId: Number,
    messageId: String,
    answer: AnswerSchema
})