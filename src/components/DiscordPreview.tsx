import { useEffect, useState } from "react";
import { Gamepad2, Clock } from "lucide-react";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import type { AppConfig, RuntimeState } from "@/types";

function activeUsername(c: AppConfig): string {
  if (c.privacyMode) return "";
  return c.roblox.accounts[c.roblox.activeAccount] || c.roblox.username || "";
}

function fmtElapsed(secs: number): string {
  secs = Math.max(0, secs);
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  return h > 0 ? `${h}h ${String(m).padStart(2, "0")}m` : `${m}m`;
}

function fmtClock(secs: number): string {
  secs = Math.max(0, secs);
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = secs % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

function resolve(
  tpl: string,
  c: AppConfig,
  rt: RuntimeState,
  nowSec: number
): string {
  const elapsed = rt.sessionStart ? Math.max(0, nowSec - rt.sessionStart) : 0;
  const map: Record<string, string> = {
    "{game}": rt.gameName ?? "",
    "{creator}": rt.creatorName ?? "",
    "{username}": activeUsername(c),
    "{placeId}": rt.placeId?.toString() ?? "",
    "{universeId}": rt.universeId?.toString() ?? "",
    "{players}": rt.playerCount?.toString() ?? "",
    "{jobId}": rt.jobId ?? "",
    "{time}": rt.sessionStart ? fmtElapsed(elapsed) : "",
  };
  let out = tpl;
  for (const [k, v] of Object.entries(map)) out = out.split(k).join(v);
  return out.trim();
}

export function DiscordPreview() {
  const t = useT();
  const config = useAppStore((s) => s.config);
  const rt = useAppStore((s) => s.runtime);
  const [now, setNow] = useState(() => Math.floor(Date.now() / 1000));

  useEffect(() => {
    const id = setInterval(() => setNow(Math.floor(Date.now() / 1000)), 1000);
    return () => clearInterval(id);
  }, []);

  if (!config) return null;

  const studio = rt.isStudio && config.roblox.detectStudio;
  const running = rt.robloxRunning || studio;
  const active = config.masterEnabled && config.discordClientId.length > 0;

  type Mode = "game" | "studio" | "menu" | "static" | null;
  let mode: Mode = null;
  if (active) {
    if (running) mode = rt.inGame ? "game" : studio ? "studio" : "menu";
    else if (config.roblox.fallbackWhenClosed === "static") mode = "static";
  }

  if (!mode) {
    return (
      <div className="discord-card">
        <div className="dc-header">{t("presence.previewTitle")}</div>
        <div className="dc-empty">
          {t("preview.empty")}
          <br />
          <span className="muted">{t("preview.emptyHint")}</span>
        </div>
      </div>
    );
  }

  const f = config.features;
  const p = config.presence;

  const templates: Record<Exclude<Mode, null>, [string, string]> = {
    game: [p.details, p.state],
    studio: ["Roblox Studio", "En train de créer"],
    menu: ["Roblox", "Dans le menu"],
    static: [config.roblox.staticDetails, config.roblox.staticState],
  };
  const [detailsTpl, stateTpl] = templates[mode];

  const details = f.showDetails ? resolve(detailsTpl, config, rt, now) : "";
  const state = f.showState ? resolve(stateTpl, config, rt, now) : "";

  let largeUrl: string | null = null;
  if (f.showLargeImage) {
    if (mode === "game" && p.largeImageMode === "auto") largeUrl = rt.gameIconUrl;
    else if (p.largeImageMode === "url") largeUrl = p.largeImageKey || null;
  }

  let smallUrl: string | null = null;
  if (f.showSmallImage) {
    if (p.smallImageMode === "url") smallUrl = p.smallImageKey || null;
    else if (p.smallImageMode === "avatar") smallUrl = rt.avatarUrl;
  }

  const timerStart =
    mode === "game" ? rt.gameStart ?? rt.sessionStart : rt.sessionStart;
  const showTimer = f.showTimer && timerStart != null && mode !== "static";
  const elapsed = timerStart ? Math.max(0, now - timerStart) : 0;

  const buttons: string[] = [];
  if (f.showButtons) {
    for (const b of p.buttons) {
      const label = resolve(b.label, config, rt, now);
      const url = resolve(b.url, config, rt, now);
      if (label && /^https?:/.test(url)) buttons.push(label);
    }
    if (f.autoButtons && mode === "game") {
      if (rt.placeId && buttons.length < 2) buttons.push("Rejoindre");
      if (rt.userId && buttons.length < 2) buttons.push("Mon profil");
    }
    buttons.splice(2);
  }

  return (
    <div className="discord-card">
      <div className="dc-header">{t("presence.previewTitle")}</div>
      <div className="dc-body">
        {f.showLargeImage && (
          <div className="dc-images">
            <div className="dc-large">
              {largeUrl ? <img src={largeUrl} alt="" /> : <Gamepad2 size={34} />}
            </div>
            {smallUrl && (
              <div className="dc-small">
                <img src={smallUrl} alt="" />
              </div>
            )}
          </div>
        )}
        <div className="dc-text">
          {details && <div className="dc-title">{details}</div>}
          {state && <div className="dc-line">{state}</div>}
          {showTimer && (
            <div className="dc-timer">
              <Clock size={13} style={{ verticalAlign: "-2px", marginRight: 5 }} />
              {fmtClock(elapsed)}
            </div>
          )}
        </div>
      </div>
      {buttons.length > 0 && (
        <div className="dc-buttons">
          {buttons.map((b, i) => (
            <div className="dc-btn" key={i}>
              {b}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
