import { useEffect, useState, type ReactElement } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { register, unregisterAll } from "@tauri-apps/plugin-global-shortcut";
import { Sidebar, type Page } from "@/components/Sidebar";
import { StatusBar } from "@/components/StatusBar";
import { DiscordConnectBanner } from "@/components/DiscordConnectBanner";
import { Presence } from "@/pages/Presence";
import { Roblox } from "@/pages/Roblox";
import { Profiles } from "@/pages/Profiles";
import { Settings } from "@/pages/Settings";
import { useAppStore } from "@/store/useAppStore";
import { onRuntimeUpdate, IS_TAURI } from "@/lib/tauri";

function hexToRgba(hex: string, alpha: number): string {
  const h = hex.replace("#", "");
  const full = h.length === 3 ? h.split("").map((c) => c + c).join("") : h;
  const n = parseInt(full, 16);
  const r = (n >> 16) & 255;
  const g = (n >> 8) & 255;
  const b = n & 255;
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

export default function App() {
  const loaded = useAppStore((s) => s.loaded);
  const load = useAppStore((s) => s.load);
  const setRuntime = useAppStore((s) => s.setRuntime);
  const config = useAppStore((s) => s.config);
  const [page, setPage] = useState<Page>("presence");

  useEffect(() => {
    load();
  }, [load]);

  useEffect(() => {
    if (!IS_TAURI) return;
    const unlisten = onRuntimeUpdate(setRuntime);
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setRuntime]);

  const theme = config?.appearance.theme;
  const accent = config?.appearance.accent;
  useEffect(() => {
    if (theme) document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);
  useEffect(() => {
    if (!accent) return;
    const root = document.documentElement.style;
    root.setProperty("--accent", accent);
    root.setProperty("--accent-glow", hexToRgba(accent, 0.35));
    root.setProperty("--accent-soft", hexToRgba(accent, 0.14));
  }, [accent]);

  const hotkey = config?.system.hotkeyToggle;
  useEffect(() => {
    if (!IS_TAURI) return;
    let cancelled = false;
    (async () => {
      try {
        await unregisterAll();
        if (hotkey && !cancelled) {
          await register(hotkey, (event) => {
            const pressed =
              typeof event === "object" ? event?.state !== "Released" : true;
            if (pressed) {
              useAppStore
                .getState()
                .updateConfig((c) => ({ ...c, masterEnabled: !c.masterEnabled }));
            }
          });
        }
      } catch (err) {
        console.error("hotkey registration failed", err);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [hotkey]);

  if (!loaded || !config) {
    return (
      <div className="setup-overlay">
        <div className="muted">…</div>
      </div>
    );
  }

  const pages: Record<Page, ReactElement> = {
    presence: <Presence />,
    roblox: <Roblox />,
    profiles: <Profiles />,
    settings: <Settings />,
  };

  return (
    <div className="app-shell">
      <Sidebar page={page} onNavigate={setPage} />
      <div className="main">
        <StatusBar />
        <div className="content">
          <DiscordConnectBanner />
          <AnimatePresence mode="wait">
            <motion.div
              key={page}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -8 }}
              transition={{ duration: 0.18 }}
            >
              {pages[page]}
            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
