// Typed wrappers around the Tauri command/event bridge.
// Tauri auto-converts camelCase JS args -> snake_case Rust params.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { AppConfig, LogEntry, RuntimeState } from "@/types";

/** True when running inside the Tauri shell (false in a plain browser preview). */
export const IS_TAURI =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export const api = {
  getConfig: () => invoke<AppConfig>("get_config"),
  setConfig: (config: AppConfig) => invoke<void>("set_config", { config }),
  getRuntime: () => invoke<RuntimeState>("get_runtime"),
  getVariables: () => invoke<string[]>("get_variables"),
  toggleMaster: () => invoke<boolean>("toggle_master"),
  reconnectDiscord: () => invoke<void>("reconnect_discord"),
  /** Resolves to the Discord application name on success; rejects with a message. */
  validateClientId: (clientId: string) =>
    invoke<string>("validate_client_id", { clientId }),
  openUrl: (url: string) => invoke<void>("open_url", { url }),
  getLogs: () => invoke<LogEntry[]>("get_logs"),
  clearLogs: () => invoke<void>("clear_logs"),
  exportConfig: (path: string) => invoke<void>("export_config", { path }),
  importConfig: (path: string) => invoke<AppConfig>("import_config", { path }),
  showMainWindow: () => invoke<void>("show_main_window"),
  quitApp: () => invoke<void>("quit_app"),
};

export function onRuntimeUpdate(
  cb: (rt: RuntimeState) => void
): Promise<UnlistenFn> {
  return listen<RuntimeState>("runtime-update", (event) => cb(event.payload));
}
