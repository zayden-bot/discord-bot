import mongoose from "mongoose";

export interface IRule {
    ruleId: string,
    ruleText: string,
    isHidden: boolean,
}

export const RulesSchema = new mongoose.Schema<IRule>({
    ruleId: String,
    ruleText: String,
    isHidden: Boolean
})
