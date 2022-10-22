interface Cron {
  time: string;
  is_open: boolean;
}

// App配置信息
interface Config {
  cron: Cron;
}

export type PluginCommanderParams = {
  "plugin:log|info": { message: string };
  "plugin:log|warn": { message: string };
  "plugin:log|error": { message: string };
};

export type PluginCommanderReturnValue = {
  "plugin:log|info": void;
  "plugin:log|warn": void;
  "plugin:log|error": void;
  "plugin:config|get": Config;
};
