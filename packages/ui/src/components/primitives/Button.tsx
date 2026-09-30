import React, { forwardRef } from "react";

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "ghost" | "outline" | "danger";
  size?: "sm" | "md" | "lg";
  iconLeft?: React.ReactNode;
  iconRight?: React.ReactNode;
  isLoading?: boolean;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      children,
      variant = "primary",
      size = "md",
      iconLeft,
      iconRight,
      isLoading = false,
      disabled,
      className = "",
      style,
      ...props
    },
    ref
  ) => {
    const baseStyle: React.CSSProperties = {
      display: "inline-flex",
      alignItems: "center",
      justifyContent: "center",
      gap: "8px",
      fontWeight: 600,
      fontFamily: "var(--nex-font-sans)",
      cursor: disabled || isLoading ? "not-allowed" : "pointer",
      opacity: disabled || isLoading ? 0.5 : 1,
      transition: "all var(--nex-transition-normal)",
      border: "1px solid transparent",
      borderRadius: "var(--nex-radius-md)",
      textDecoration: "none",
      userSelect: "none",
      whiteSpace: "nowrap",
      letterSpacing: "0.01em",
      ...style,
    };

    const sizeStyles: Record<"sm" | "md" | "lg", React.CSSProperties> = {
      sm: { padding: "6px 12px", fontSize: "0.8rem", height: "32px" },
      md: { padding: "10px 18px", fontSize: "0.9rem", height: "40px" },
      lg: { padding: "14px 26px", fontSize: "1rem", height: "48px" },
    };

    const variantStyles: Record<
      "primary" | "secondary" | "ghost" | "outline" | "danger",
      React.CSSProperties
    > = {
      primary: {
        backgroundColor: "var(--nex-accent)",
        color: "#000000",
        boxShadow: "var(--nex-shadow-sm)",
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
      outline: {
        backgroundColor: "transparent",
        color: "var(--nex-text-primary)",
        borderColor: "var(--nex-border-strong)",
      },
      danger: {
        backgroundColor: "rgba(239, 68, 68, 0.15)",
        color: "var(--nex-error)",
        borderColor: "rgba(239, 68, 68, 0.3)",
      },
    };

    return (
      <button
        ref={ref}
        disabled={disabled || isLoading}
        style={{
          ...baseStyle,
          ...sizeStyles[size],
          ...variantStyles[variant],
        }}
        className={`nex-button nex-button-${variant} nex-button-${size} ${className}`}
        {...props}
      >
        {isLoading && (
          <span
            style={{
              width: "14px",
              height: "14px",
              border: "2px solid currentColor",
              borderRightColor: "transparent",
              borderRadius: "50%",
              display: "inline-block",
              animation: "spin 0.6s linear infinite",
            }}
          />
        )}
        {!isLoading && iconLeft}
        {children}
        {!isLoading && iconRight}
      </button>
    );
  }
);

Button.displayName = "Button";
