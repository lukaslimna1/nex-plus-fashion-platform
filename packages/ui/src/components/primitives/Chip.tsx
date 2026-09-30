import React from "react";

export interface ChipProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  active?: boolean;
  size?: "sm" | "md";
  icon?: React.ReactNode;
}

export const Chip: React.FC<ChipProps> = ({
  children,
  active = false,
  size = "md",
  icon,
  className = "",
  style,
  ...props
}) => {
  const sizeStyle: React.CSSProperties =
    size === "sm"
      ? { padding: "4px 10px", fontSize: "0.78rem" }
      : { padding: "6px 14px", fontSize: "0.85rem" };

  return (
    <button
      type="button"
      style={{
        display: "inline-flex",
        alignItems: "center",
        gap: "6px",
        borderRadius: "var(--nex-radius-full)",
        border: "1px solid",
        borderColor: active ? "var(--nex-accent)" : "var(--nex-border-subtle)",
        backgroundColor: active ? "var(--nex-accent)" : "var(--nex-surface-1)",
        color: active ? "#000000" : "var(--nex-text-primary)",
        fontWeight: active ? 600 : 500,
        cursor: "pointer",
        transition: "all var(--nex-transition-normal)",
        userSelect: "none",
        whiteSpace: "nowrap",
        ...sizeStyle,
        ...style,
      }}
      className={`nex-chip ${active ? "nex-chip-active" : ""} ${className}`}
      {...props}
    >
      {icon}
      {children}
    </button>
  );
};

export interface TagProps {
  children: React.ReactNode;
  variant?: "default" | "gold" | "subtle";
  className?: string;
  style?: React.CSSProperties;
}

export const Tag: React.FC<TagProps> = ({
  children,
  variant = "default",
  className = "",
  style,
}) => {
  const variantStyles: Record<"default" | "gold" | "subtle", React.CSSProperties> = {
    default: {
      backgroundColor: "var(--nex-surface-2)",
      color: "var(--nex-text-secondary)",
      border: "1px solid var(--nex-border-subtle)",
    },
    gold: {
      backgroundColor: "rgba(212, 175, 55, 0.1)",
      color: "var(--nex-accent-gold)",
      border: "1px solid rgba(212, 175, 55, 0.3)",
    },
    subtle: {
      backgroundColor: "transparent",
      color: "var(--nex-text-muted)",
      border: "1px solid var(--nex-border-subtle)",
    },
  };

  return (
    <span
      style={{
        display: "inline-flex",
        alignItems: "center",
        padding: "3px 8px",
        borderRadius: "var(--nex-radius-sm)",
        fontSize: "0.75rem",
        fontWeight: 500,
        lineHeight: 1.2,
        ...variantStyles[variant],
        ...style,
      }}
      className={`nex-tag nex-tag-${variant} ${className}`}
    >
      {children}
    </span>
  );
};
