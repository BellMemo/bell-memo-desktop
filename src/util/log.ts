import { invoke } from "@tauri-apps/api";

const info = (message: string) => {
  console.log(`BellMemo Info: ${message}`);
  invoke("plugin:log|info", {
    message,
  });
};

/**
 * console.
 * @param message
 */
const warn = (message: string) => {
  console.log(`BellMemo Warn: ${message}`);
  invoke("plugin:log|warn", {
    message,
  });
};

const error = (message: string) => {
  console.log(`BellMemo Error: ${message}`);
  invoke("plugin:log|error", {
    message,
  });
};

export const Log = {
  info,
  warn,
  error,
};
