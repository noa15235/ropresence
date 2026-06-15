import { Trash2, Plus, Gamepad2 } from "lucide-react";
import { Section } from "@/components/GlassPanel";
import { Field } from "@/components/Field";
import { Toggle } from "@/components/Toggle";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import type { RobloxConfig } from "@/types";

export function Roblox() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const rt = useAppStore((s) => s.runtime);
  const update = useAppStore((s) => s.updateConfig);

  const r = config.roblox;
  const setR = (patch: Partial<RobloxConfig>) =>
    update((c) => ({ ...c, roblox: { ...c.roblox, ...patch } }));

  return (
    <div className="stack">
      <Section title={t("roblox.title")}>
        <Field label={t("roblox.username")}>
          <input
            className="input"
            value={r.username}
            placeholder={t("roblox.usernamePh")}
            onChange={(e) => setR({ username: e.target.value })}
          />
        </Field>

        <div className="feature-row">
          <div className="feature-text">
            <span className="feature-name">{t("roblox.privacy")}</span>
            <span className="feature-desc">{t("roblox.privacyDesc")}</span>
          </div>
          <Toggle
            checked={config.privacyMode}
            ariaLabel={t("roblox.privacy")}
            onChange={(v) => update((c) => ({ ...c, privacyMode: v }))}
          />
        </div>
        <div className="feature-row">
          <div className="feature-text">
            <span className="feature-name">{t("roblox.detectStudio")}</span>
            <span className="feature-desc">{t("roblox.detectStudioDesc")}</span>
          </div>
          <Toggle
            checked={r.detectStudio}
            ariaLabel={t("roblox.detectStudio")}
            onChange={(v) => setR({ detectStudio: v })}
          />
        </div>
      </Section>

      <Section title={t("roblox.accounts")}>
        <div className="stack">
          {r.accounts.map((acc, i) => (
            <div className="list-item" key={i}>
              <input
                className="input"
                value={acc}
                placeholder={t("roblox.accountPh")}
                onChange={(e) => {
                  const accounts = [...r.accounts];
                  accounts[i] = e.target.value;
                  setR({ accounts });
                }}
              />
              <button
                className="btn btn-icon btn-ghost"
                aria-label={t("common.remove")}
                onClick={() => {
                  const accounts = r.accounts.filter((_, idx) => idx !== i);
                  const activeAccount = Math.min(
                    r.activeAccount,
                    Math.max(0, accounts.length - 1)
                  );
                  setR({ accounts, activeAccount });
                }}
              >
                <Trash2 size={16} />
              </button>
            </div>
          ))}
          <button
            className="btn btn-sm"
            style={{ alignSelf: "flex-start" }}
            onClick={() => setR({ accounts: [...r.accounts, ""] })}
          >
            <Plus size={15} />
            {t("roblox.addAccount")}
          </button>
        </div>

        {r.accounts.length > 0 && (
          <Field label={t("roblox.activeAccount")} >
            <select
              className="select"
              value={r.activeAccount}
              onChange={(e) => setR({ activeAccount: Number(e.target.value) })}
            >
              {r.accounts.map((acc, i) => (
                <option key={i} value={i}>
                  {acc || `#${i + 1}`}
                </option>
              ))}
            </select>
          </Field>
        )}
      </Section>

      <Section title={t("roblox.fallback")}>
        <Field>
          <select
            className="select"
            value={r.fallbackWhenClosed}
            onChange={(e) =>
              setR({ fallbackWhenClosed: e.target.value as "clear" | "static" })
            }
          >
            <option value="clear">{t("roblox.fallbackClear")}</option>
            <option value="static">{t("roblox.fallbackStatic")}</option>
          </select>
        </Field>

        {r.fallbackWhenClosed === "static" && (
          <div className="grid-2">
            <Field label={t("roblox.staticDetails")}>
              <input
                className="input"
                value={r.staticDetails}
                onChange={(e) => setR({ staticDetails: e.target.value })}
              />
            </Field>
            <Field label={t("roblox.staticState")}>
              <input
                className="input"
                value={r.staticState}
                onChange={(e) => setR({ staticState: e.target.value })}
              />
            </Field>
          </div>
        )}

        <Field label={t("roblox.poll")} hint={`${r.pollIntervalSecs} ${t("roblox.pollDesc")}`}>
          <input
            type="range"
            min={2}
            max={15}
            value={r.pollIntervalSecs}
            onChange={(e) => setR({ pollIntervalSecs: Number(e.target.value) })}
          />
        </Field>
      </Section>

      {(rt.robloxRunning || rt.isStudio) && (
        <Section title="Roblox">
          <div className="row" style={{ gap: 14 }}>
            <div className="dc-large" style={{ width: 64, height: 64 }}>
              {rt.gameIconUrl ? (
                <img src={rt.gameIconUrl} alt="" />
              ) : (
                <Gamepad2 size={26} />
              )}
            </div>
            <div className="stack" style={{ gap: 2 }}>
              <div className="feature-name">
                {rt.gameName ?? (rt.isStudio ? t("status.studio") : t("status.menu"))}
              </div>
              {rt.creatorName && (
                <div className="feature-desc">{rt.creatorName}</div>
              )}
              {rt.placeId && (
                <div className="muted" style={{ fontSize: 11 }}>
                  placeId {rt.placeId}
                  {rt.playerCount != null && ` · ${rt.playerCount} joueurs`}
                </div>
              )}
            </div>
          </div>
        </Section>
      )}
    </div>
  );
}
