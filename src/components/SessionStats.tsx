import { useEffect, useState } from "react";
import { Gamepad2, Clock, CalendarClock } from "lucide-react";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";

function fmtClock(secs: number): string {
  secs = Math.max(0, secs);
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = secs % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

function fmtHm(secs: number): string {
  secs = Math.max(0, secs);
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  return h > 0 ? `${h}h ${String(m).padStart(2, "0")}m` : `${m}m`;
}

export function SessionStats() {
  const t = useT();
  const rt = useAppStore((s) => s.runtime);
  const [now, setNow] = useState(() => Math.floor(Date.now() / 1000));

  useEffect(() => {
    const id = setInterval(() => setNow(Math.floor(Date.now() / 1000)), 1000);
    return () => clearInterval(id);
  }, []);

  if (!rt.robloxRunning && !rt.isStudio) return null;

  const game = rt.inGame && rt.gameStart ? now - rt.gameStart : null;
  const session = rt.sessionStart ? now - rt.sessionStart : null;
  const daily = rt.dailySeconds ?? 0;

  return (
    <span className="session-stats">
      {game != null && (
        <span title={t("stats.game")}>
          <Gamepad2 size={13} />
          {fmtClock(game)}
        </span>
      )}
      {session != null && (
        <span title={t("stats.session")}>
          <Clock size={13} />
          {fmtClock(session)}
        </span>
      )}
      <span title={t("stats.today")}>
        <CalendarClock size={13} />
        {fmtHm(daily)}
      </span>
    </span>
  );
}
