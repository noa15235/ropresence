import { Trash2, Plus } from "lucide-react";
import { Section } from "@/components/GlassPanel";
import { Field } from "@/components/Field";
import { FeatureToggle } from "@/components/FeatureToggle";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { BUTTON_PRESETS } from "@/lib/presets";
import type { PresenceButton, PresenceConfig } from "@/types";

export function Buttons() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const update = useAppStore((s) => s.updateConfig);

  const p = config.presence;
  const setButtons = (buttons: PresenceButton[]) =>
    update((c) => ({
      ...c,
      presence: { ...c.presence, buttons } as PresenceConfig,
    }));

  const canAdd = p.buttons.length < 2;

  const addPreset = (label: string, url: string) => {
    if (p.buttons.length >= 2 || p.buttons.some((b) => b.url === url)) return;
    setButtons([...p.buttons, { label, url }]);
  };

  return (
    <div className="stack">
      <Section title={t("buttons.customButtons")}>
        <p className="note" style={{ marginBottom: 14 }}>
          {t("buttons.maxNote")}
        </p>

        <div className="label" style={{ marginBottom: 8 }}>
          {t("buttons.quickAdd")}
        </div>
        <div className="preset-row" style={{ marginBottom: 16 }}>
          {BUTTON_PRESETS.map((bp) => (
            <button
              key={bp.id}
              className="btn btn-sm"
              disabled={!canAdd || p.buttons.some((b) => b.url === bp.url)}
              onClick={() => addPreset(t(bp.labelKey), bp.url)}
            >
              <Plus size={14} />
              {t(bp.labelKey)}
            </button>
          ))}
        </div>
        <div className="stack">
          {p.buttons.map((b, i) => (
            <div className="glass card" key={i} style={{ padding: 14 }}>
              <div className="row-between" style={{ marginBottom: 10 }}>
                <span className="feature-name">#{i + 1}</span>
                <button
                  className="btn btn-icon btn-ghost"
                  aria-label={t("common.remove")}
                  onClick={() => setButtons(p.buttons.filter((_, idx) => idx !== i))}
                >
                  <Trash2 size={16} />
                </button>
              </div>
              <div className="grid-2">
                <Field label={t("buttons.buttonLabel")}>
                  <input
                    className="input"
                    value={b.label}
                    placeholder={t("buttons.labelPh")}
                    onChange={(e) => {
                      const next = [...p.buttons];
                      next[i] = { ...next[i], label: e.target.value };
                      setButtons(next);
                    }}
                  />
                </Field>
                <Field label={t("buttons.buttonUrl")}>
                  <input
                    className="input"
                    value={b.url}
                    placeholder={t("buttons.urlPh")}
                    onChange={(e) => {
                      const next = [...p.buttons];
                      next[i] = { ...next[i], url: e.target.value };
                      setButtons(next);
                    }}
                  />
                </Field>
              </div>
            </div>
          ))}

          <button
            className="btn btn-sm"
            style={{ alignSelf: "flex-start" }}
            disabled={!canAdd}
            onClick={() => setButtons([...p.buttons, { label: "", url: "" }])}
          >
            <Plus size={15} />
            {t("buttons.addButton")}
          </button>
        </div>
      </Section>

      <Section title={t("buttons.title")}>
        <FeatureToggle featureKey="showButtons" />
        <FeatureToggle featureKey="autoButtons" desc={t("buttons.autoButtonsDesc")} />
      </Section>
    </div>
  );
}
