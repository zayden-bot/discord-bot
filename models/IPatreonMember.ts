export interface IPatreonMember {
    attributes: IPatreonMemberAttributes
    id: string
    type: string
}

interface IPatreonMemberAttributes {
    campaign_lifetime_support_cents: number
    email: string
    patron_status: string
}
