import { useEffect, useState } from "react";
import {
  Download,
  Upload,
  Trash2,
  Power,
  RefreshCw,
  ExternalLink,
  Plug,
} from "lucide-react";
import {
  enable as enableAutostart,
  disable as disableAutostart,
  isEnabled as isAutostartEnabled,
} from "@tauri-apps/plugin-autostart";
import { save, open } from "@tauri-apps/plugin-dialog";
import { Section } from "@/components/GlassPanel";
import { Field } from "@/components/Field";
import { Toggle } from "@/components/Toggle";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { api, IS_TAURI } from "@/lib/tauri";
import type { Appearance, LogEntry, SystemConfig } from "@/types";

const PORTAL_URL = "https://discord.com/developers/applications";

const ACCENT_PRESETS = ["#2E9BFF", "#7C5CFF", "#FF5C8A", "#3BA55D", "#FAA61A", "#FF6B3D"];

export function Settings() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const update = useAppStore((s) => s.updateConfig);
  const replaceConfig = useAppStore((s) => s.replaceConfig);
  const [logs, setLogs] = useState<LogEntry[]>([]);

  const sys = config.system;
  const setSys = (patch: Partial<SystemConfig>) =>
    update((c) => ({ ...c, system: { ...c.system, ...patch } }));
  const setLook = (patch: Partial<Appearance>) =>
    update((c) => ({ ...c, appearance: { ...c.appearance, ...patch } }));

  // Keep the autostart toggle in sync with the OS reality on mount.
  useEffect(() => {
    isAutostartEnabled()
      .then((en) => {
        if (en !== config.system.autostart) setSys({ autostart: en });
      })
      .catch(() => {});
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const refreshLogs = () => api.getLogs().then(setLogs).catch(() => {});
  useEffect(() => {
    refreshLogs();
    const id = setInterval(refreshLogs, 4000);
    return () => clearInterval(id);
  }, []);

  async function toggleAutostart(value: boolean) {
    try {
      if (value) await enableAutostart();
      else await disableAutostart();
      setSys({ autostart: value });
    } catch (e) {
      console.error("autostart", e);
    }
  }

  async function exportConfig() {
    const path = await save({
      defaultPath: "ropresence-config.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (path) await api.exportConfig(path);
  }

  async function importConfig() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (typeof selected === "string") {
      const cfg = await api.importConfig(selected);
      replaceConfig(cfg);
    }
  }

  return (
    <div className="stack">
      <DiscordSection />

      <Section title={t("settings.appearance")}>
        <div className="grid-2">
          <Field label={t("settings.theme")}>
            <select
              className="select"
              value={config.appearance.theme}
              onChange={(e) =>
                setLook({ theme: e.target.value as "dark" | "light" })
              }
            >
              <option value="dark">{t("settings.themeDark")}</option>
              <option value="light">{t("settings.themeLight")}</option>
            </select>
          </Field>
          <Field label={t("settings.language")}>
            <select
              className="select"
              value={config.appearance.language}
              onChange={(e) =>
                setLook({ language: e.target.value as "fr" | "en" })
              }
            >
              <option value="fr">Français</option>
              <option value="en">English</option>
            </select>
          </Field>
        </div>

        <Field label={t("settings.accent")}>
          <div className="row">
            <div className="swatches">
              {ACCENT_PRESETS.map((color) => (
                <button
                  key={color}
                  className={`swatch ${
                    config.appearance.accent.toLowerCase() === color.toLowerCase()
                      ? "active"
                      : ""
                  }`}
                  style={{ background: color }}
                  onClick={() => setLook({ accent: color })}
                  aria-label={color}
                />
              ))}
            </div>
            <input
              type="color"
              value={config.appearance.accent}
              onChange={(e) => setLook({ accent: e.target.value })}
              style={{
                width: 36,
                height: 30,
                border: "none",
                background: "none",
                cursor: "pointer",
              }}
            />
          </div>
        </Field>
      </Section>

      <Section title={t("settings.system")}>
        <SettingRow
          name={t("settings.autostart")}
          desc={t("settings.autostartDesc")}
          checked={sys.autostart}
          onChange={toggleAutostart}
        />
        <SettingRow
          name={t("settings.startMinimized")}
          desc={t("settings.startMinimizedDesc")}
          checked={sys.startMinimized}
          onChange={(v) => setSys({ startMinimized: v })}
        />
        <SettingRow
          name={t("settings.closeToTray")}
          desc={t("settings.closeToTrayDesc")}
          checked={sys.closeToTray}
          onChange={(v) => setSys({ closeToTray: v })}
        />
        <SettingRow
          name={t("settings.notifications")}
          desc={t("settings.notificationsDesc")}
          checked={sys.notifications}
          onChange={(v) => setSys({ notifications: v })}
        />
        <Field
          label={t("settings.hotkey")}
          hint={t("settings.hotkeyDesc")}
          ok={undefined}
        >
          <input
            className="input"
            value={sys.hotkeyToggle}
            placeholder={t("settings.hotkeyPh")}
            onChange={(e) => setSys({ hotkeyToggle: e.target.value })}
          />
        </Field>
      </Section>

      <Section title={t("settings.data")}>
        <div className="row">
          <button className="btn" onClick={exportConfig}>
            <Download size={16} />
            {t("settings.export")}
          </button>
          <button className="btn" onClick={importConfig}>
            <Upload size={16} />
            {t("settings.import")}
          </button>
        </div>
      </Section>

      <Section
        title={t("settings.debug")}
        right={
          <div className="row">
            <button className="btn btn-sm btn-ghost" onClick={refreshLogs}>
              <RefreshCw size={14} />
            </button>
            <button
              className="btn btn-sm btn-ghost"
              onClick={() => api.clearLogs().then(refreshLogs)}
            >
              <Trash2 size={14} />
              {t("settings.clearLogs")}
            </button>
          </div>
        }
      >
        {logs.length === 0 ? (
          <div className="muted" style={{ fontSize: 12.5 }}>
            {t("settings.noLogs")}
          </div>
        ) : (
          <div className="logs">
            {logs
              .slice()
              .reverse()
              .map((l, i) => (
                <div className="log-line" key={i}>
                  <span className={`lv-${l.level}`}>
                    {l.level.toUpperCase()}
                  </span>
                  <span className="log-scope">[{l.scope}]</span>
                  <span>{l.message}</span>
                </div>
              ))}
          </div>
        )}
      </Section>

      <Section title="">
        <button className="btn btn-danger" onClick={() => api.quitApp()}>
          <Power size={16} />
          {t("settings.quit")}
        </button>
      </Section>
    </div>
  );
}

function DiscordSection() {
  const t = useT();
  const clientId = useAppStore((s) => s.config?.discordClientId ?? "");
  const connected = useAppStore((s) => s.runtime.discordConnected);
  const update = useAppStore((s) => s.updateConfig);
  const [val, setVal] = useState(clientId);
  const [status, setStatus] = useState<{ ok: boolean; msg: string } | null>(null);
  const [checking, setChecking] = useState(false);

  async function validateAndConnect() {
    setChecking(true);
    setStatus(null);
    try {
      const name = await api.validateClientId(val);
      update((c) => ({ ...c, discordClientId: val.trim(), masterEnabled: true }));
      setStatus({ ok: true, msg: `${t("setup.valid")} · ${name}` });
      if (IS_TAURI) {
        // Give the debounced save a moment, then force an immediate reconnect.
        setTimeout(() => api.reconnectDiscord().catch(() => {}), 400);
      }
    } catch (e) {
      setStatus({ ok: false, msg: String(e) });
    } finally {
      setChecking(false);
    }
  }

  return (
    <Section title={t("settings.discord")}>
      <Field label={t("setup.clientIdLabel")} hint={t("setup.clientIdHelp")}>
        <input
          className="input"
          value={val}
          placeholder={t("setup.clientIdPlaceholder")}
          onChange={(e) => {
            setVal(e.target.value);
            setStatus(null);
          }}
        />
        {status && (
          <span className={status.ok ? "field-ok" : "field-error"}>{status.msg}</span>
        )}
      </Field>
      <div className="row">
        <button
          className="btn btn-primary"
          disabled={checking || !val.trim()}
          onClick={validateAndConnect}
        >
          <Plug size={15} />
          {checking ? t("connect.connecting") : t("settings.validateConnect")}
        </button>
        <button className="btn" onClick={() => api.openUrl(PORTAL_URL)}>
          <ExternalLink size={15} />
          {t("setup.openPortal")}
        </button>
        <span className="spacer" />
        <span className="pill">
          <span className={`dot ${connected ? "ok" : "off"}`} />
          {connected ? t("status.connected") : t("status.disconnected")}
        </span>
      </div>
      <p className="note" style={{ marginTop: 10 }}>
        {t("connect.privacyHint")}
      </p>
    </Section>
  );
}

function SettingRow({
  name,
  desc,
  checked,
  onChange,
}: {
  name: string;
  desc: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <div className="feature-row">
      <div className="feature-text">
        <span className="feature-name">{name}</span>
        <span className="feature-desc">{desc}</span>
      </div>
      <Toggle checked={checked} onChange={onChange} ariaLabel={name} />
    </div>
  );
}
