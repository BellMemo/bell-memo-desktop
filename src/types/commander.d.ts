//  定义项目所有可调用命令的入参
//  入参结构以Object为标准，统一为 `{params: any}`
export type CommanderParams = {
  select_memo_data: { params: { content: string } };
};

//  定义项目所有可调用命令的出参
export type CommanderReturnValue = {
  select_memo_data: void;
};
