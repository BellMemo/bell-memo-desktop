import { PluginCommanderParams, PluginCommanderReturnValue } from "./plugin";
import { CommanderParams, CommanderReturnValue } from "./commander";

export type CommonReturnValue = CommanderReturnValue &
  PluginCommanderReturnValue;

export type CommonParams = CommanderParams & PluginCommanderParams;

export * from "./model";
