import type { AppConfig } from "@/types";

export interface PresencePreset {
  id: string;
  labelKey: string;
  apply: (c: AppConfig) => AppConfig;
}

function patch(
  c: AppConfig,
  presence: Partial<AppConfig["presence"]>,
  features: Partial<AppConfig["features"]>,
  extra?: Partial<AppConfig>
): AppConfig {
  return {
    ...c,
    ...extra,
    presence: { ...c.presence, ...presence },
    features: { ...c.features, ...features },
  };
}

const EXPERIENCE_BTN = {
  label: "Voir l'expérience",
  url: "https://www.roblox.com/games/{placeId}",
};
const PROFILE_BTN = {
  label: "Mon profil Roblox",
  url: "https://www.roblox.com/users/{userId}/profile",
};

export const PRESENCE_PRESETS: PresencePreset[] = [
  {
    id: "detailed",
    labelKey: "presets.detailed",
    apply: (c) =>
      patch(
        c,
        {
          details: "{game}",
          state: "par {creator}",
          largeImageMode: "auto",
          largeImageText: "{game}",
          smallImageMode: "avatar",
          smallImageText: "{username}",
          buttons: [EXPERIENCE_BTN, PROFILE_BTN],
        },
        {
          showDetails: true,
          showState: true,
          showTimer: true,
          showLargeImage: true,
          showSmallImage: true,
          showButtons: true,
          autoButtons: false,
        },
        { privacyMode: false }
      ),
  },
  {
    id: "simple",
    labelKey: "presets.simple",
    apply: (c) =>
      patch(
        c,
        {
          details: "{game}",
          state: "par {creator}",
          largeImageMode: "auto",
          largeImageText: "{game}",
          smallImageMode: "none",
          buttons: [],
        },
        {
          showDetails: true,
          showState: true,
          showTimer: true,
          showLargeImage: true,
          showSmallImage: false,
          showButtons: false,
          autoButtons: false,
        },
        { privacyMode: false }
      ),
  },
  {
    id: "streamer",
    labelKey: "presets.streamer",
    apply: (c) =>
      patch(
        c,
        {
          details: "🎮 {game}",
          state: "{players} joueurs en ligne",
          largeImageMode: "auto",
          largeImageText: "{game}",
          smallImageMode: "avatar",
          smallImageText: "{username}",
          buttons: [EXPERIENCE_BTN],
        },
        {
          showDetails: true,
          showState: true,
          showTimer: true,
          showLargeImage: true,
          showSmallImage: true,
          showButtons: true,
          showParty: true,
          autoButtons: false,
        },
        { privacyMode: false }
      ),
  },
  {
    id: "discreet",
    labelKey: "presets.discreet",
    apply: (c) =>
      patch(
        c,
        {
          details: "Joue à Roblox",
          state: "",
          largeImageMode: "auto",
          largeImageText: "Roblox",
          smallImageMode: "none",
          buttons: [],
        },
        {
          showDetails: true,
          showState: false,
          showTimer: true,
          showLargeImage: true,
          showSmallImage: false,
          showButtons: false,
          autoButtons: false,
        },
        { privacyMode: true }
      ),
  },
];

export interface ButtonPreset {
  id: string;
  labelKey: string;
  label: string;
  url: string;
}

export const BUTTON_PRESETS: ButtonPreset[] = [
  {
    id: "experience",
    labelKey: "buttons.presetExperience",
    label: EXPERIENCE_BTN.label,
    url: EXPERIENCE_BTN.url,
  },
  {
    id: "profile",
    labelKey: "buttons.presetProfile",
    label: PROFILE_BTN.label,
    url: PROFILE_BTN.url,
  },
  {
    id: "join",
    labelKey: "buttons.presetJoin",
    label: "Rejoindre",
    url: "https://www.roblox.com/games/{placeId}",
  },
];
