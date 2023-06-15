export interface ConversationModel {
    id: string;
    title: string;
    history: {
        role: string;
        content: string;
    }[];
}