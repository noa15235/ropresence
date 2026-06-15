import { useState } from "react";
import { Trash2, Check, Layers } from "lucide-react";
import { Section } from "@/components/GlassPanel";
import { Field } from "@/components/Field";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import type { PresenceProfile } from "@/types";

export function Profiles() {
  const t = useT();
  const config = useAppStore((s) => s.config)!;
  const update = useAppStore((s) => s.updateConfig);
  const [name, setName] = useState("");

  function createProfile() {
    const trimmed = name.trim();
    if (!trimmed) return;
    const profile: PresenceProfile = {
      id: crypto.randomUUID(),
      name: trimmed,
      presence: structuredClone(config.presence),
      features: structuredClone(config.features),
    };
    update((c) => ({
      ...c,
      profiles: [...c.profiles, profile],
      activeProfile: profile.id,
    }));
    setName("");
  }

  function applyProfile(profile: PresenceProfile) {
    update((c) => ({
      ...c,
      presence: structuredClone(profile.presence),
      features: structuredClone(profile.features),
      activeProfile: profile.id,
    }));
  }

  function deleteProfile(id: string) {
    update((c) => ({
      ...c,
      profiles: c.profiles.filter((pr) => pr.id !== id),
      activeProfile: c.activeProfile === id ? "" : c.activeProfile,
    }));
  }

  return (
    <div className="stack">
      <Section title={t("profiles.newProfile")}>
        <div className="row" style={{ alignItems: "flex-end" }}>
          <div style={{ flex: 1 }}>
            <Field label={t("profiles.profileName")}>
              <input
                className="input"
                value={name}
                placeholder={t("profiles.namePh")}
                onChange={(e) => setName(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && createProfile()}
              />
            </Field>
          </div>
          <button
            className="btn btn-primary"
            disabled={!name.trim()}
            onClick={createProfile}
          >
            {t("profiles.saveCurrent")}
          </button>
        </div>
      </Section>

      <Section title={t("profiles.title")}>
        {config.profiles.length === 0 ? (
          <div className="dc-empty" style={{ color: "var(--text-muted)" }}>
            {t("profiles.noneYet")}
          </div>
        ) : (
          <div className="stack">
            {config.profiles.map((pr) => {
              const isActive = pr.id === config.activeProfile;
              return (
                <div className="list-item" key={pr.id}>
                  <Layers size={16} className="muted" />
                  <span className="feature-name" style={{ flex: 1 }}>
                    {pr.name}
                    {isActive && (
                      <span className="muted" style={{ marginLeft: 8, fontSize: 11 }}>
                        · {t("profiles.current")}
                      </span>
                    )}
                  </span>
                  <button
                    className="btn btn-sm"
                    onClick={() => applyProfile(pr)}
                    disabled={isActive}
                  >
                    <Check size={14} />
                    {t("profiles.apply")}
                  </button>
                  <button
                    className="btn btn-icon btn-ghost"
                    aria-label={t("common.delete")}
                    onClick={() => deleteProfile(pr.id)}
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              );
            })}
          </div>
        )}
      </Section>
    </div>
  );
}
