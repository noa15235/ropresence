import fr from "./fr.json";
import en from "./en.json";
import { useAppStore } from "@/store/useAppStore";

const dict = { fr, en } as const;
type Lang = keyof typeof dict;

type Tree = { [k: string]: string | Tree };

function lookup(obj: Tree, path: string): string | undefined {
  const value = path
    .split(".")
    .reduce<string | Tree | undefined>(
      (acc, k) => (acc && typeof acc === "object" ? acc[k] : undefined),
      obj
    );
  return typeof value === "string" ? value : undefined;
}

export function translate(
  lang: Lang,
  key: string,
  vars?: Record<string, string | number>
): string {
  let str = lookup(dict[lang] as Tree, key) ?? lookup(dict.fr as Tree, key) ?? key;
  if (vars) {
    for (const [k, v] of Object.entries(vars)) {
      str = str.replace(new RegExp(`\\{${k}\\}`, "g"), String(v));
    }
  }
  return str;
}

/** Hook returning a `t(key, vars?)` bound to the current language. */
export function useT() {
  const lang = (useAppStore((s) => s.config?.appearance.language) ?? "fr") as Lang;
  return (key: string, vars?: Record<string, string | number>) =>
    translate(lang, key, vars);
}
