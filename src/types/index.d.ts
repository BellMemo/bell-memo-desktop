import { PluginCommanderParams, PluginCommanderReturnValue } from "./plugin";
import { CommanderParams, CommanderReturnValue } from "./commander";

type CommonReturnValue = CommanderReturnValue & PluginCommanderReturnValue;
type CommonParams = CommanderParams & PluginCommanderParams;

type InvokeType<T extends keyof CommonReturnValue> =
  T extends keyof CommonParams
    ? (cmd: T, args: CommonParams[T]) => Promise<CommonReturnValue[T]>
    : (cmd: T) => Promise<CommonReturnValue[T]>;

type UnionToIntersection<U> = (
  U extends U ? (x: U) => unknown : never
) extends (x: infer R) => unknown
  ? R
  : never;

declare module "@tauri-apps/api" {
  declare const invoke: UnionToIntersection<
    InvokeType<keyof CommonReturnValue>
  >;
}

export * from "./model";

