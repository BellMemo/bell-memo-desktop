export interface MemoData {
  id: string;
  title: string;
  content: string;
  created: number;
  updated: number;
}

export interface MemoTag {
  id: string;
  name: string;
  created: number;
  updated: number;
}

export interface MemoTagData {
  id: string;
  memo_id: string;
  tag_id: string;
  created: number;
  updated: number;
}
