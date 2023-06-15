import { writable, type Writable } from 'svelte/store';
import type { ConversationModel } from './models';

export const viewConversation: Writable<null|ConversationModel> = writable(null);