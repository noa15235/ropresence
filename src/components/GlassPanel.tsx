import type { ReactNode } from "react";

interface Props {
  children: ReactNode;
  className?: string;
}

/** A translucent, blurred glass surface (section 5 design system). */
export function GlassPanel({ children, className = "" }: Props) {
  return <div className={`glass ${className}`}>{children}</div>;
}

interface SectionProps {
  title?: string;
  children: ReactNode;
  right?: ReactNode;
}

/** A titled glass card. */
export function Section({ title, children, right }: SectionProps) {
  return (
    <section className="glass card">
      {(title || right) && (
        <div className="row-between" style={{ marginBottom: 14 }}>
          {title && (
            <div className="section-title" style={{ marginBottom: 0 }}>
              {title}
            </div>
          )}
          {right}
        </div>
      )}
      {children}
    </section>
  );
}
