type InvokeType<T extends keyof CommanderReturnValue> =
  T extends keyof CommanderParams
    ? (cmd: T, args: CommanderParams[T]) => Promise<CommanderReturnValue[T]>
    : (cmd: T) => Promise<CommanderReturnValue[T]>;

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

//  定义项目所有可调用命令的入参
type CommanderParams = {
  log: string;
};

//  定义项目所有可调用命令的出参
type CommanderReturnValue = {
  get_greet: void;
  log: void;
};
