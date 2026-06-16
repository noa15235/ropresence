import { useState } from "react";
import { motion } from "framer-motion";
import { ExternalLink, Check } from "lucide-react";
import { GlassPanel } from "@/components/GlassPanel";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { api } from "@/lib/tauri";

const PORTAL_URL = "https://discord.com/developers/applications";

export function Setup() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const update = useAppStore((s) => s.updateConfig);

  const [value, setValue] = useState(config.discordClientId);
  const [error, setError] = useState<string | null>(null);
  const [valid, setValid] = useState(false);
  const [appName, setAppName] = useState<string | null>(null);
  const [checking, setChecking] = useState(false);

  async function validate() {
    setChecking(true);
    try {
      const name = await api.validateClientId(value);
      setAppName(name);
      setValid(true);
      setError(null);
    } catch (e) {
      setValid(false);
      setAppName(null);
      setError(String(e));
    } finally {
      setChecking(false);
    }
  }

  function finish() {
    update((c) => ({ ...c, discordClientId: value.trim(), setupComplete: true }));
  }

  return (
    <div className="setup-overlay">
      <motion.div
        initial={{ opacity: 0, y: 18, scale: 0.98 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        transition={{ type: "spring", stiffness: 220, damping: 24 }}
        style={{ width: "100%", maxWidth: 560 }}
      >
        <GlassPanel className="setup-card">
          <div className="page-title">{t("setup.title")}</div>
          <div className="page-sub">{t("setup.subtitle")}</div>
          <p className="note" style={{ marginTop: 16 }}>
            {t("setup.intro")}
          </p>

          <div className="section-title" style={{ marginTop: 22 }}>
            {t("setup.instructionsTitle")}
          </div>
          <ol className="setup-steps">
            {[1, 2, 3, 4].map((n) => (
              <li key={n}>
                <span className="step-num">{n}</span>
                <span>{t(`setup.instr${n}`)}</span>
              </li>
            ))}
          </ol>
          <button className="btn btn-sm" onClick={() => api.openUrl(PORTAL_URL)}>
            <ExternalLink size={14} />
            {t("setup.openPortal")}
          </button>

          <div className="field" style={{ marginTop: 20 }}>
            <label className="label">
              {t("setup.clientIdLabel")}
              <span className="hint">{t("setup.clientIdHelp")}</span>
            </label>
            <input
              className="input"
              value={value}
              placeholder={t("setup.clientIdPlaceholder")}
              onChange={(e) => {
                setValue(e.target.value);
                setValid(false);
                setAppName(null);
                setError(null);
              }}
            />
            {error && <span className="field-error">{error}</span>}
            {valid && (
              <span className="field-ok">
                <Check size={12} style={{ verticalAlign: "-2px" }} /> {t("setup.valid")}
                {appName ? ` · ${appName}` : ""}
              </span>
            )}
          </div>

          <p className="note">{t("setup.assetsNote")}</p>

          <div className="row" style={{ marginTop: 18, justifyContent: "flex-end" }}>
            <button
              className="btn btn-ghost"
              onClick={() => update((c) => ({ ...c, setupComplete: true }))}
            >
              {t("setup.skip")}
            </button>
            <button className="btn" onClick={validate} disabled={checking || !value.trim()}>
              {checking ? t("connect.connecting") : t("setup.validate")}
            </button>
            <button
              className="btn btn-primary"
              disabled={!value.trim()}
              onClick={finish}
            >
              {t("setup.finish")}
            </button>
          </div>
        </GlassPanel>
      </motion.div>
    </div>
  );
}
