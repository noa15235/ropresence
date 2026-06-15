import { useState, type FocusEvent } from "react";
import { GlassPanel, Section } from "@/components/GlassPanel";
import { Field } from "@/components/Field";
import { FeatureToggle } from "@/components/FeatureToggle";
import { DiscordPreview } from "@/components/DiscordPreview";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import { PRESENCE_PRESETS } from "@/lib/presets";
import type { ImageMode, PresenceConfig } from "@/types";

type TextFieldKey = "details" | "state" | "largeImageText" | "smallImageText";

export function Presence() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const variables = useAppStore((s) => s.variables);
  const update = useAppStore((s) => s.updateConfig);

  const p = config.presence;
  const [active, setActive] = useState<{
    key: TextFieldKey;
    el: HTMLInputElement;
  } | null>(null);

  const setP = (patch: Partial<PresenceConfig>) =>
    update((c) => ({ ...c, presence: { ...c.presence, ...patch } }));

  function insertVariable(token: string) {
    if (!active) return;
    const { key, el } = active;
    const cur = p[key];
    const start = el.selectionStart ?? cur.length;
    const end = el.selectionEnd ?? start;
    const next = cur.slice(0, start) + token + cur.slice(end);
    setP({ [key]: next } as Partial<PresenceConfig>);
    requestAnimationFrame(() => {
      el.focus();
      const pos = start + token.length;
      el.setSelectionRange(pos, pos);
    });
  }

  const trackFocus = (key: TextFieldKey) => (e: FocusEvent<HTMLInputElement>) =>
    setActive({ key, el: e.currentTarget });

  return (
    <div className="layout-split">
      <div className="stack">
        <Section title={t("presence.presetsTitle")}>
          <p className="note" style={{ marginBottom: 12 }}>
            {t("presence.presetsDesc")}
          </p>
          <div className="preset-row">
            {PRESENCE_PRESETS.map((preset) => (
              <button
                key={preset.id}
                className="btn btn-sm"
                onClick={() => update(preset.apply)}
              >
                {t(preset.labelKey)}
              </button>
            ))}
          </div>
        </Section>

        <Section title={t("presence.title")}>
          <Field label={t("presence.detailsLabel")}>
            <input
              className="input"
              value={p.details}
              placeholder={t("presence.detailsPh")}
              onFocus={trackFocus("details")}
              onChange={(e) => setP({ details: e.target.value })}
            />
          </Field>
          <Field label={t("presence.stateLabel")}>
            <input
              className="input"
              value={p.state}
              placeholder={t("presence.statePh")}
              onFocus={trackFocus("state")}
              onChange={(e) => setP({ state: e.target.value })}
            />
          </Field>

          <div className="label" style={{ marginTop: 6 }}>
            {t("presence.variablesTitle")}
            <span className="hint">{t("presence.variablesDesc")}</span>
          </div>
          <div className="chips" style={{ marginTop: 8 }}>
            {variables.map((v) => (
              <button key={v} className="chip" onClick={() => insertVariable(v)}>
                {v}
              </button>
            ))}
          </div>
        </Section>

        <Section title={t("presence.images")}>
          <div className="grid-2">
            <ImageBlock
              title={t("presence.largeImage")}
              mode={p.largeImageMode}
              modeOptions={["auto", "asset", "url"]}
              imageKey={p.largeImageKey}
              hoverText={p.largeImageText}
              onMode={(m) => setP({ largeImageMode: m })}
              onKey={(k) => setP({ largeImageKey: k })}
              onHover={(h) => setP({ largeImageText: h })}
              onHoverFocus={trackFocus("largeImageText")}
              t={t}
            />
            <ImageBlock
              title={t("presence.smallImage")}
              mode={p.smallImageMode}
              modeOptions={["none", "avatar", "asset", "url"]}
              imageKey={p.smallImageKey}
              hoverText={p.smallImageText}
              onMode={(m) => setP({ smallImageMode: m })}
              onKey={(k) => setP({ smallImageKey: k })}
              onHover={(h) => setP({ smallImageText: h })}
              onHoverFocus={trackFocus("smallImageText")}
              t={t}
            />
          </div>
        </Section>

        <Section title={t("presence.display")}>
          <FeatureToggle featureKey="showDetails" />
          <FeatureToggle featureKey="showState" />
          <FeatureToggle featureKey="showTimer" desc={t("presence.timerDesc")} />
          <FeatureToggle featureKey="showLargeImage" />
          <FeatureToggle featureKey="showSmallImage" />
        </Section>
      </div>

      <div className="discord-preview-wrap">
        <GlassPanel className="card">
          <DiscordPreview />
        </GlassPanel>
      </div>
    </div>
  );
}

interface ImageBlockProps {
  title: string;
  mode: ImageMode;
  modeOptions: ImageMode[];
  imageKey: string;
  hoverText: string;
  onMode: (m: ImageMode) => void;
  onKey: (k: string) => void;
  onHover: (h: string) => void;
  onHoverFocus: (e: FocusEvent<HTMLInputElement>) => void;
  t: (key: string) => string;
}

function ImageBlock({
  title,
  mode,
  modeOptions,
  imageKey,
  hoverText,
  onMode,
  onKey,
  onHover,
  onHoverFocus,
  t,
}: ImageBlockProps) {
  const modeLabel: Record<ImageMode, string> = {
    auto: t("presence.modeAuto"),
    asset: t("presence.modeAsset"),
    url: t("presence.modeUrl"),
    none: t("presence.modeNone"),
    avatar: t("presence.modeAvatar"),
  };

  return (
    <div>
      <div className="label" style={{ marginBottom: 8 }}>
        {title}
      </div>
      <Field>
        <select
          className="select"
          value={mode}
          onChange={(e) => onMode(e.target.value as ImageMode)}
        >
          {modeOptions.map((m) => (
            <option key={m} value={m}>
              {modeLabel[m]}
            </option>
          ))}
        </select>
      </Field>

      {mode === "asset" && (
        <Field label={t("presence.assetKey")}>
          <input
            className="input"
            value={imageKey}
            placeholder={t("presence.assetKeyPh")}
            onChange={(e) => onKey(e.target.value)}
          />
        </Field>
      )}
      {mode === "url" && (
        <Field label={t("presence.imageUrl")}>
          <input
            className="input"
            value={imageKey}
            placeholder={t("presence.imageUrlPh")}
            onChange={(e) => onKey(e.target.value)}
          />
        </Field>
      )}

      {mode !== "none" && (
        <Field label={t("presence.hoverText")}>
          <input
            className="input"
            value={hoverText}
            placeholder={t("presence.hoverTextPh")}
            onFocus={onHoverFocus}
            onChange={(e) => onHover(e.target.value)}
          />
        </Field>
      )}
    </div>
  );
}
