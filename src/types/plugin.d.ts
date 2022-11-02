interface Cron {
  time: string;
  is_open: boolean;
  immediately: boolean;
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
  /**
   * 启动定时任务
   */
  "plugin:timer|set_task": void;
  /**
   * 关闭定时任务
   */
  "plugin:timer|stop_task": void;
};
