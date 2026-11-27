export type Conversation = {
  reservationId: string;
  reference: string;
  propertyNameEn: string;
  propertyNameFr: string;
  timezone: string;
  lastBody: string;
  lastAt: string;
  unreadCount: number;
};

export type ConversationPage = { items: Conversation[]; page: number; pageSize: number; total: number };
