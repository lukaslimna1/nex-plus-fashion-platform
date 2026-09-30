import React, { useState } from "react";

export interface TooltipProps {
  content: React.ReactNode;
  children: React.ReactElement;
  position?: "top" | "bottom" | "left" | "right";
}

export const Tooltip: React.FC<TooltipProps> = ({
  content,
  children,
  position = "top",
}) => {
  const [isVisible, setIsVisible] = useState(false);

  const positionStyles: Record<"top" | "bottom" | "left" | "right", React.CSSProperties> = {
    top: {
      bottom: "calc(100% + 8px)",
      left: "50%",
      transform: "translateX(-50%)",
    },
    bottom: {
      top: "calc(100% + 8px)",
      left: "50%",
      transform: "translateX(-50%)",
    },
    left: {
      right: "calc(100% + 8px)",
      top: "50%",
      transform: "translateY(-50%)",
    },
    right: {
      left: "calc(100% + 8px)",
      top: "50%",
      transform: "translateY(-50%)",
    },
  };

  return (
    <div
      style={{ position: "relative", display: "inline-flex" }}
      onMouseEnter={() => setIsVisible(true)}
      onMouseLeave={() => setIsVisible(false)}
      onFocus={() => setIsVisible(true)}
      onBlur={() => setIsVisible(false)}
    >
      {children}
      {isVisible && (
        <div
          role="tooltip"
          style={{
            position: "absolute",
            zIndex: "var(--nex-z-toast)",
            backgroundColor: "var(--nex-surface-2)",
            color: "var(--nex-text-primary)",
            padding: "5px 10px",
            borderRadius: "var(--nex-radius-sm)",
            fontSize: "0.75rem",
            fontWeight: 500,
            whiteSpace: "nowrap",
            boxShadow: "var(--nex-shadow-md)",
            border: "1px solid var(--nex-border-strong)",
            pointerEvents: "none",
            ...positionStyles[position],
          }}
          className="nex-tooltip"
        >
          {content}
        </div>
      )}
    </div>
  );
};
