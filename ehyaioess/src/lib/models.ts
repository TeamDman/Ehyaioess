export interface ConversationModel {
    id: string;
    title: string;
    history: {
        id: string;
        author: string;
        content: string;
    }[];
}