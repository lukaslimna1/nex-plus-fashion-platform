import React, { forwardRef } from "react";

export interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "ghost" | "glass";
  size?: "sm" | "md" | "lg";
  ariaLabel: string;
}

export const IconButton = forwardRef<HTMLButtonElement, IconButtonProps>(
  (
    {
      children,
      variant = "ghost",
      size = "md",
      ariaLabel,
      disabled,
      className = "",
      style,
      ...props
    },
    ref
  ) => {
    const sizeMap = {
      sm: { width: "32px", height: "32px", fontSize: "14px" },
      md: { width: "40px", height: "40px", fontSize: "18px" },
      lg: { width: "48px", height: "48px", fontSize: "22px" },
    };

    const variantStyles: Record<"primary" | "secondary" | "ghost" | "glass", React.CSSProperties> = {
      primary: {
        backgroundColor: "var(--nex-accent)",
        color: "#000000",
      },
      secondary: {
        backgroundColor: "var(--nex-surface-2)",
        color: "var(--nex-text-primary)",
        borderColor: "var(--nex-border-subtle)",
      },
      ghost: {
        backgroundColor: "transparent",
        color: "var(--nex-text-primary)",
      },
      glass: {
        backgroundColor: "var(--nex-surface-elevated)",
        backdropFilter: "blur(var(--nex-blur-md))",
        color: "var(--nex-text-primary)",
        borderColor: "var(--nex-border-subtle)",
      },
    };

    return (
      <button
        ref={ref}
        aria-label={ariaLabel}
        disabled={disabled}
        style={{
          display: "inline-flex",
          alignItems: "center",
          justifyContent: "center",
          borderRadius: "var(--nex-radius-full)",
          cursor: disabled ? "not-allowed" : "pointer",
          opacity: disabled ? 0.4 : 1,
          border: "1px solid transparent",
          transition: "all var(--nex-transition-normal)",
          ...sizeMap[size],
          ...variantStyles[variant],
          ...style,
        }}
        className={`nex-icon-button nex-icon-button-${variant} ${className}`}
        {...props}
      >
        {children}
      </button>
    );
  }
);

IconButton.displayName = "IconButton";
