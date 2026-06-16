import { Toggle } from "./Toggle";
import { useAppStore } from "@/store/useAppStore";
import { useT } from "@/i18n";
import type { FeatureKey } from "@/types";

interface Props {
  featureKey: FeatureKey;
  desc?: string;
}

export function FeatureToggle({ featureKey, desc }: Props) {
  const t = useT();
  const value = useAppStore((s) => s.config?.features[featureKey] ?? false);
  const update = useAppStore((s) => s.updateConfig);

  return (
    <div className="feature-row">
      <div className="feature-text">
        <span className="feature-name">{t(`features.${featureKey}`)}</span>
        {desc && <span className="feature-desc">{desc}</span>}
      </div>
      <Toggle
        checked={value}
        ariaLabel={t(`features.${featureKey}`)}
        onChange={(v) =>
          update((c) => ({ ...c, features: { ...c.features, [featureKey]: v } }))
        }
      />
    </div>
  );
}
