import { Radio, Gamepad2, Link2, Layers, Settings as SettingsIcon } from "lucide-react";
import { useT } from "@/i18n";

export type Page = "presence" | "roblox" | "buttons" | "profiles" | "settings";

interface Props {
  page: Page;
  onNavigate: (page: Page) => void;
}

export function Sidebar({ page, onNavigate }: Props) {
  const t = useT();

  const items: { id: Page; icon: typeof Radio; label: string }[] = [
    { id: "presence", icon: Radio, label: t("nav.presence") },
    { id: "roblox", icon: Gamepad2, label: t("nav.roblox") },
    { id: "buttons", icon: Link2, label: t("nav.buttons") },
    { id: "profiles", icon: Layers, label: t("nav.profiles") },
    { id: "settings", icon: SettingsIcon, label: t("nav.settings") },
  ];

  return (
    <aside className="sidebar">
      <div className="brand">
        <div className="brand-logo">
          <Gamepad2 size={20} />
        </div>
        <div>
          <div className="brand-name">{t("app.name")}</div>
          <div className="brand-tag">{t("app.tagline")}</div>
        </div>
      </div>

      <nav className="nav">
        {items.map((item) => {
          const Icon = item.icon;
          const active = page === item.id;
          return (
            <button
              key={item.id}
              className={`nav-item ${active ? "active" : ""}`}
              onClick={() => onNavigate(item.id)}
            >
              <Icon size={17} />
              {item.label}
            </button>
          );
        })}
      </nav>

      <div className="sidebar-foot">
        <span>v0.1.0</span>
      </div>
    </aside>
  );
}
