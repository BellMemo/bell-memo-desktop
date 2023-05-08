import { create } from "zustand";

interface ContextType {
  loading: boolean;
  setLoading: (v: boolean) => void;

  searchContent: string;
  setSearchContent: (v: string) => void;
}

export const useAppStore = create<ContextType>((set) => ({
  loading: false,
  setLoading: (v) => {
    set(() => ({ loading: v }));
  },

  searchContent: "",
  setSearchContent(v) {
    set(() => ({ searchContent: v }));
  },
}));
