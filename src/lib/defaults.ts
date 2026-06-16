import type { AppConfig, RuntimeState } from "@/types";

export const DEFAULT_CLIENT_ID = "363445589247131668";

export const DEFAULT_VARIABLES = [
  "{game}",
  "{creator}",
  "{username}",
  "{userId}",
  "{placeId}",
  "{universeId}",
  "{players}",
  "{jobId}",
  "{time}",
];

export function defaultConfig(): AppConfig {
  return {
    discordClientId: DEFAULT_CLIENT_ID,
    masterEnabled: true,
    privacyMode: false,
    setupComplete: true,
    presence: {
      details: "{game}",
      state: "par {creator}",
      largeImageMode: "auto",
      largeImageKey: "roblox",
      largeImageText: "{game}",
      smallImageMode: "none",
      smallImageKey: "",
      smallImageText: "{username}",
      buttons: [],
    },
    roblox: {
      username: "",
      accounts: [],
      activeAccount: 0,
      detectStudio: true,
      fallbackWhenClosed: "clear",
      staticDetails: "Sur le bureau",
      staticState: "En attente de Roblox…",
      pollIntervalSecs: 4,
    },
    features: {
      showDetails: true,
      showState: true,
      showTimer: true,
      showLargeImage: true,
      showSmallImage: true,
      showButtons: true,
      showParty: false,
      autoButtons: true,
    },
    appearance: { theme: "dark", accent: "#2E9BFF", language: "fr" },
    system: {
      autostart: false,
      startMinimized: false,
      closeToTray: true,
      notifications: true,
      hotkeyToggle: "",
    },
    profiles: [],
    activeProfile: "",
  };
}

export function demoRuntime(): RuntimeState {
  return {
    discordConnected: true,
    robloxRunning: true,
    inGame: true,
    isStudio: false,
    placeId: 920587237,
    universeId: 245662005,
    jobId: "demo-instance",
    gameName: "Adopt Me!",
    creatorName: "Uplift Games",
    gameIconUrl: null,
    avatarUrl: null,
    userId: 1,
    playerCount: 124530,
    maxPlayers: 48,
    sessionStart: Math.floor(Date.now() / 1000) - 7325,
    gameStart: Math.floor(Date.now() / 1000) - 612,
    dailySeconds: 9420,
    lastError: null,
  };
}
