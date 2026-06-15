import type { ReactNode } from "react";

interface Props {
  label?: string;
  hint?: string;
  error?: string;
  ok?: string;
  children: ReactNode;
}

/** A labelled form field wrapper with optional hint/validation text. */
export function Field({ label, hint, error, ok, children }: Props) {
  return (
    <div className="field">
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
