import { CommonParams, CommonReturnValue } from "@src/types";
import { invoke } from "@tauri-apps/api";

/**
 * 通用封装
 * @TODO 支持远端API接口适配
 */
export const request: <T extends keyof CommonReturnValue>(
  ...params: T extends keyof CommonParams ? [T, CommonParams[T]] : [T]
) => Promise<CommonReturnValue[T]> = (async (cmd, args) => {
  return invoke(cmd, args);
}) as any;
