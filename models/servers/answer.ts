import mongoose from "mongoose";

export const AnswerSchema = new mongoose.Schema({
    text: String,
    userId: String,
})