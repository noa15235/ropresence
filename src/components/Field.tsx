import type { CSSProperties, ReactNode } from "react";

interface Props {
  label?: string;
  hint?: string;
  error?: string;
  ok?: string;
  style?: CSSProperties;
  children: ReactNode;
}

export function Field({ label, hint, error, ok, style, children }: Props) {
  return (
    <div className="field" style={style}>
      {label && (
        <label className="label">
          {label}
          {hint && <span className="hint">{hint}</span>}
        </label>
      )}
      {children}
      {error && <span className="field-error">{error}</span>}
      {ok && <span className="field-ok">{ok}</span>}
    </div>
  );
}
