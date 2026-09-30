import React, { useState, useRef, useEffect } from "react";

export interface MenuItem {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode | undefined;
  onClick?: (() => void) | undefined;
  danger?: boolean | undefined;
  disabled?: boolean | undefined;
  divider?: boolean | undefined;
}

export interface DropdownProps {
  trigger: React.ReactNode;
  items: MenuItem[];
  align?: "left" | "right" | undefined;
  className?: string | undefined;
}

export const Dropdown: React.FC<DropdownProps> = ({
  trigger,
  items,
  align = "right",
  className = "",
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setIsOpen(false);
      }
    };
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") setIsOpen(false);
    };

    if (isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("keydown", handleKeyDown);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen]);

  return (
    <div
      ref={containerRef}
      style={{ position: "relative", display: "inline-block" }}
      className={`nex-dropdown ${className}`}
    >
      <div onClick={() => setIsOpen((prev) => !prev)} style={{ cursor: "pointer" }}>
        {trigger}
      </div>

      {isOpen && (
        <div
          role="menu"
          style={{
            position: "absolute",
            top: "calc(100% + 6px)",
            [align]: 0,
            zIndex: "var(--nex-z-header)",
            minWidth: "200px",
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-strong)",
            borderRadius: "var(--nex-radius-md)",
            boxShadow: "var(--nex-shadow-lg)",
            padding: "6px",
            display: "flex",
            flexDirection: "column",
            gap: "2px",
            backdropFilter: "blur(var(--nex-blur-lg))",
          }}
        >
          {items.map((item, idx) => {
            if (item.divider) {
              return (
                <div
                  key={`div-${idx}`}
                  style={{
                    height: "1px",
                    backgroundColor: "var(--nex-border-subtle)",
                    margin: "4px 0",
                  }}
                />
              );
            }
            return (
              <button
                key={item.id}
                role="menuitem"
                disabled={item.disabled}
                onClick={() => {
                  if (item.disabled) return;
                  item.onClick?.();
                  setIsOpen(false);
                }}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "10px",
                  padding: "8px 12px",
                  fontSize: "0.85rem",
                  fontWeight: 500,
                  color: item.danger
                    ? "var(--nex-error)"
                    : item.disabled
                    ? "var(--nex-text-muted)"
                    : "var(--nex-text-primary)",
                  backgroundColor: "transparent",
                  border: "none",
                  borderRadius: "var(--nex-radius-sm)",
                  cursor: item.disabled ? "not-allowed" : "pointer",
                  textAlign: "left",
                  width: "100%",
                  transition: "background var(--nex-transition-fast)",
                }}
                onMouseEnter={(e) => {
                  if (!item.disabled) {
                    e.currentTarget.style.backgroundColor = "var(--nex-surface-2)";
                  }
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor = "transparent";
                }}
              >
                {item.icon && <span style={{ opacity: 0.8 }}>{item.icon}</span>}
                <span style={{ flex: 1 }}>{item.label}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
};
