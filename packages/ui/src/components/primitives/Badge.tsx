import React from "react";

export interface BadgeProps {
  children: React.ReactNode;
  variant?: "live" | "verified" | "canonical" | "warning" | "tier" | "neutral";
  size?: "sm" | "md";
  className?: string;
  style?: React.CSSProperties;
}

export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = "neutral",
  size = "md",
  className = "",
  style,
}) => {
  const sizeStyle: React.CSSProperties =
    size === "sm"
      ? { padding: "2px 6px", fontSize: "0.68rem" }
      : { padding: "4px 8px", fontSize: "0.75rem" };

  const variantStyles: Record<
    "live" | "verified" | "canonical" | "warning" | "tier" | "neutral",
    React.CSSProperties
  > = {
    live: {
      backgroundColor: "rgba(239, 68, 68, 0.16)",
      color: "#ff4d4d",
      border: "1px solid rgba(239, 68, 68, 0.4)",
      boxShadow: "var(--nex-shadow-glow-live)",
    },
    verified: {
      backgroundColor: "rgba(16, 185, 129, 0.14)",
      color: "#34d399",
      border: "1px solid rgba(16, 185, 129, 0.35)",
    },
    canonical: {
      backgroundColor: "rgba(255, 255, 255, 0.08)",
      color: "var(--nex-text-primary)",
      border: "1px solid var(--nex-border-strong)",
    },
    warning: {
      backgroundColor: "rgba(245, 158, 11, 0.15)",
      color: "#fbbf24",
      border: "1px solid rgba(245, 158, 11, 0.35)",
    },
    tier: {
      backgroundColor: "rgba(212, 175, 55, 0.15)",
      color: "var(--nex-accent-gold)",
      border: "1px solid rgba(212, 175, 55, 0.4)",
    },
    neutral: {
      backgroundColor: "var(--nex-surface-2)",
      color: "var(--nex-text-secondary)",
      border: "1px solid var(--nex-border-subtle)",
    },
  };

  return (
    <span
      style={{
        display: "inline-flex",
        alignItems: "center",
        gap: "5px",
        fontWeight: 600,
        textTransform: "uppercase",
        letterSpacing: "0.06em",
        borderRadius: "var(--nex-radius-sm)",
        lineHeight: 1,
        userSelect: "none",
        ...sizeStyle,
        ...variantStyles[variant],
        ...style,
      }}
      className={`nex-badge nex-badge-${variant} ${className}`}
    >
      {variant === "live" && (
        <span
          style={{
            width: "6px",
            height: "6px",
            borderRadius: "50%",
            backgroundColor: "var(--nex-live)",
            display: "inline-block",
            boxShadow: "0 0 6px var(--nex-live)",
          }}
        />
      )}
      {children}
    </span>
  );
};
