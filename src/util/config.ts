import { invoke } from "@tauri-apps/api";

const get = () => {
  return invoke("plugin:config|get");
};

export const AppConfig = {
  get,
};
