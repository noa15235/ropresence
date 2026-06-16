import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Plug, Loader2, AlertCircle } from "lucide-react";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { api, IS_TAURI } from "@/lib/tauri";
import { DEFAULT_CLIENT_ID } from "@/lib/defaults";

export function DiscordConnectBanner() {
  const t = useT();
  const connected = useAppStore((s) => s.runtime.discordConnected);
  const lastError = useAppStore((s) => s.runtime.lastError);
  const update = useAppStore((s) => s.updateConfig);
  const [busy, setBusy] = useState(false);

  async function quickConnect() {
    if (!IS_TAURI) return;
    setBusy(true);
    update((c) => ({
      ...c,
      discordClientId: DEFAULT_CLIENT_ID,
      masterEnabled: true,
    }));
    setTimeout(() => api.reconnectDiscord().catch(() => {}), 450);
    setTimeout(() => setBusy(false), 3000);
  }

  return (
    <AnimatePresence>
      {!connected && (
        <motion.div
          className="connect-banner"
          initial={{ opacity: 0, y: -10, height: 0 }}
          animate={{ opacity: 1, y: 0, height: "auto" }}
          exit={{ opacity: 0, y: -10, height: 0 }}
          transition={{ type: "spring", stiffness: 300, damping: 30 }}
        >
          <div className="connect-banner-icon">
            <Plug size={18} />
          </div>
          <div className="connect-banner-text">
            <div className="connect-banner-title">{t("connect.title")}</div>
            <div className="connect-banner-sub">
              {t("connect.quickSub")}
              {lastError && (
                <span className="connect-banner-err">
                  <AlertCircle size={12} style={{ verticalAlign: "-2px" }} />{" "}
                  {lastError}
                </span>
              )}
            </div>
            <div className="connect-banner-hint">{t("connect.privacyHint")}</div>
          </div>
          <button
            className="btn btn-primary"
            disabled={busy}
            onClick={quickConnect}
          >
            {busy ? (
              <>
                <Loader2 size={16} className="spin" />
                {t("connect.connecting")}
              </>
            ) : (
              <>
                <Plug size={16} />
                {t("connect.quick")}
              </>
            )}
          </button>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
