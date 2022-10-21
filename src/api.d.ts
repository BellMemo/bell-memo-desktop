type InvokeType<T extends keyof CommanderReturnValue> =
  T extends keyof CommanderParams
    ? (cmd: T, args: CommanderParams[T]) => CommanderReturnValue[T]
    : (cmd: T) => CommanderReturnValue[T];

type UnionToIntersection<U> = (
  U extends U ? (x: U) => unknown : never
) extends (x: infer R) => unknown
  ? R
  : never;

declare module "@tauri-apps/api" {
  declare const invoke: UnionToIntersection<
    InvokeType<keyof CommanderReturnValue>
  >;
}

interface Cron {
  time: string;
  is_open: boolean;
}
// App配置信息
interface Config {
  cron: Cron;
}

//  定义项目所有可调用命令的入参
//  入参结构以Object为标准，统一为 `{params: any}`
type CommanderParams = {
  "plugin:log|info": { message: string };
  "plugin:log|warn": { message: string };
  "plugin:log|error": { message: string };
  select_memo_data: { params: { content: string } };
};

//  定义项目所有可调用命令的出参
type CommanderReturnValue = {
  "plugin:log|info": void;
  "plugin:log|warn": void;
  "plugin:log|error": void;
  "plugin:config|get": Config;
  select_memo_data: void;
};
