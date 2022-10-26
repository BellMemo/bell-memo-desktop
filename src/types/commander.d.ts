import { MemoData, MemoTag } from "./model";

//  定义项目所有可调用命令的入参
//  入参结构以Object为标准，统一为 `{params: any}`
export type CommanderParams = {
  select_memo_data: { params: { content: string } };
  search_memo_tag: {
    params: { content: string; offset: number; limit: number };
  };
  insert_memo_tag: {
    params: { content: string };
  };
  insert_memo_data: {
    params: { title: string; tags: string[]; content: string };
  };
};

//  定义项目所有可调用命令的出参
export type CommanderReturnValue = {
  select_memo_data: Array<
    Omit<MemoData, "created" | "updated"> & {
      tags: {
        id: string;
        name: string;
      };
    }
  >;
  search_memo_tag: Array<MemoTag>;
  insert_memo_tag: MemoTag;
  insert_memo_data: void;
};
