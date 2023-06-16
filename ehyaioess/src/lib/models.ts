export type AuthorRole = 'system' | 'assistant' | 'user'; 
export interface ConversationMessage {
    conversation_id: string;
    id: string;
    author: AuthorRole;
    content: string;
}
export interface Conversation {
    id: string;
    title: string;
    history: ConversationMessage[];
}