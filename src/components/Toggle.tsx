import { motion } from "framer-motion";

interface Props {
  checked: boolean;
  onChange: (value: boolean) => void;
  ariaLabel?: string;
  disabled?: boolean;
}

export function Toggle({ checked, onChange, ariaLabel, disabled }: Props) {
  return (
    <motion.button
      type="button"
      className={`toggle ${checked ? "on" : ""}`}
      aria-pressed={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      onClick={() => onChange(!checked)}
      whileTap={{ scale: 0.92 }}
    >
      <motion.span
        className="toggle-knob"
        animate={{ x: checked ? 18 : 0 }}
        transition={{ type: "spring", stiffness: 600, damping: 34 }}
      />
    </motion.button>
  );
}
