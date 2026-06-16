import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { api, IS_TAURI } from "@/lib/tauri";
import { Toggle } from "./Toggle";
import { SessionStats } from "./SessionStats";

export function StatusBar() {
  const t = useT();
  const rt = useAppStore((s) => s.runtime);
  const master = useAppStore((s) => s.config?.masterEnabled ?? false);
  const update = useAppStore((s) => s.updateConfig);

  let robloxDot = "off";
  let robloxText = t("status.noGame");
  if (rt.inGame) {
    robloxDot = "ok";
    robloxText = t("status.inGame");
  } else if (rt.isStudio) {
    robloxDot = "ok";
    robloxText = t("status.studio");
  } else if (rt.robloxRunning) {
    robloxDot = "warn";
    robloxText = t("status.menu");
  }

  const reconnect = () => {
    if (IS_TAURI && !rt.discordConnected) api.reconnectDiscord().catch(() => {});
  };

  return (
    <div className="statusbar">
      <span
        className={`pill ${!rt.discordConnected ? "clickable" : ""}`}
        onClick={reconnect}
        title={!rt.discordConnected ? t("connect.button") : undefined}
      >
        <span className={`dot ${rt.discordConnected ? "ok" : "off"}`} />
        {t("status.discord")} ·{" "}
        {rt.discordConnected ? t("status.connected") : t("status.disconnected")}
      </span>

      <span className="pill">
        <span className={`dot ${robloxDot}`} />
        {t("status.roblox")} · {robloxText}
      </span>

      <span className="statusbar-spacer" />

      <SessionStats />

      <span className="master">
        {master ? t("status.masterOn") : t("status.masterOff")}
        <Toggle
          checked={master}
          ariaLabel={t("status.master")}
          onChange={(v) => update((c) => ({ ...c, masterEnabled: v }))}
        />
      </span>
    </div>
  );
}
