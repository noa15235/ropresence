import { create } from "zustand";
import type { AppConfig, RuntimeState } from "@/types";
import { api, IS_TAURI } from "@/lib/tauri";
import { defaultConfig, demoRuntime, DEFAULT_VARIABLES } from "@/lib/defaults";

const emptyRuntime: RuntimeState = {
  discordConnected: false,
  robloxRunning: false,
  inGame: false,
  isStudio: false,
  placeId: null,
  universeId: null,
  jobId: null,
  gameName: null,
  creatorName: null,
  gameIconUrl: null,
  avatarUrl: null,
  userId: null,
  playerCount: null,
  maxPlayers: null,
  sessionStart: null,
  gameStart: null,
  dailySeconds: null,
  lastError: null,
};

interface AppStore {
  config: AppConfig | null;
  runtime: RuntimeState;
  variables: string[];
  loaded: boolean;
  load: () => Promise<void>;
  setRuntime: (rt: RuntimeState) => void;
  updateConfig: (updater: (c: AppConfig) => AppConfig) => void;
  replaceConfig: (c: AppConfig) => void;
}

let saveTimer: ReturnType<typeof setTimeout> | null = null;
function scheduleSave(config: AppConfig) {
  if (!IS_TAURI) return;
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    api.setConfig(config).catch((e) => console.error("setConfig failed", e));
  }, 300);
}

export const useAppStore = create<AppStore>((set, get) => ({
  config: null,
  runtime: emptyRuntime,
  variables: [],
  loaded: false,

  load: async () => {
    if (!IS_TAURI) {
      set({
        config: defaultConfig(),
        runtime: demoRuntime(),
        variables: DEFAULT_VARIABLES,
        loaded: true,
      });
      return;
    }
    const fallback = setTimeout(() => {
      console.warn("load() timed out — showing UI with defaults");
      set({
        config: defaultConfig(),
        runtime: demoRuntime(),
        variables: DEFAULT_VARIABLES,
        loaded: true,
      });
    }, 6000);
    try {
      const [config, runtime, variables] = await Promise.all([
        api.getConfig(),
        api.getRuntime(),
        api.getVariables(),
      ]);
      clearTimeout(fallback);
      set({ config, runtime, variables, loaded: true });
    } catch (e) {
      clearTimeout(fallback);
      console.warn("Tauri backend unavailable — preview mode", e);
      set({
        config: defaultConfig(),
        runtime: demoRuntime(),
        variables: DEFAULT_VARIABLES,
        loaded: true,
      });
    }
  },

  setRuntime: (rt) => set({ runtime: rt }),

  updateConfig: (updater) => {
    const current = get().config;
    if (!current) return;
    const next = updater(structuredClone(current));
    set({ config: next });
    scheduleSave(next);
  },

  replaceConfig: (c) => {
    set({ config: c });
    scheduleSave(c);
  },
}));
