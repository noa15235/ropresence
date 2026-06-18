export interface PresenceButton {
  label: string;
  url: string;
}

export interface FeatureFlags {
  showDetails: boolean;
  showState: boolean;
  showTimer: boolean;
  showLargeImage: boolean;
  showSmallImage: boolean;
  showButtons: boolean;
  showParty: boolean;
  autoButtons: boolean;
}

export type ImageMode = "auto" | "asset" | "url" | "none" | "avatar";

export type TimerMode = "auto" | "session" | "game" | "off";

export interface PresenceConfig {
  details: string;
  state: string;
  largeImageMode: ImageMode;
  largeImageKey: string;
  largeImageText: string;
  smallImageMode: ImageMode;
  smallImageKey: string;
  smallImageText: string;
  buttons: PresenceButton[];
  menuDetails: string;
  menuState: string;
  studioDetails: string;
  studioState: string;
  timerMode: TimerMode;
}

export interface RobloxConfig {
  username: string;
  accounts: string[];
  activeAccount: number;
  detectStudio: boolean;
  fallbackWhenClosed: "clear" | "static";
  staticDetails: string;
  staticState: string;
  pollIntervalSecs: number;
}

export interface Appearance {
  theme: "dark" | "light";
  accent: string;
  language: "fr" | "en";
}

export interface SystemConfig {
  autostart: boolean;
  startMinimized: boolean;
  closeToTray: boolean;
  notifications: boolean;
  hotkeyToggle: string;
}

export interface PresenceProfile {
  id: string;
  name: string;
  presence: PresenceConfig;
  features: FeatureFlags;
}

export interface AppConfig {
  discordClientId: string;
  masterEnabled: boolean;
  privacyMode: boolean;
  setupComplete: boolean;
  presence: PresenceConfig;
  roblox: RobloxConfig;
  features: FeatureFlags;
  appearance: Appearance;
  system: SystemConfig;
  profiles: PresenceProfile[];
  activeProfile: string;
}

export interface RuntimeState {
  discordConnected: boolean;
  robloxRunning: boolean;
  inGame: boolean;
  isStudio: boolean;
  placeId: number | null;
  universeId: number | null;
  jobId: string | null;
  gameName: string | null;
  creatorName: string | null;
  gameIconUrl: string | null;
  avatarUrl: string | null;
  userId: number | null;
  playerCount: number | null;
  maxPlayers: number | null;
  sessionStart: number | null;
  gameStart: number | null;
  dailySeconds: number | null;
  lastError: string | null;
}

export type LogLevel = "info" | "warn" | "error";

export interface LogEntry {
  ts: number;
  level: LogLevel;
  scope: string;
  message: string;
}

export type FeatureKey = keyof FeatureFlags;
