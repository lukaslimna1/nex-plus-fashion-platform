import React, { useEffect, useRef } from "react";
import { X } from "lucide-react";

export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: React.ReactNode;
  children: React.ReactNode;
  maxWidth?: string;
}

export const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  children,
  maxWidth = "560px",
}) => {
  const modalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    if (isOpen) {
      document.body.style.overflow = "hidden";
      window.addEventListener("keydown", handleKeyDown);
    }
    return () => {
      document.body.style.overflow = "";
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div
      role="dialog"
      aria-modal="true"
      style={{
        position: "fixed",
        inset: 0,
        zIndex: "var(--nex-z-modal)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        padding: "20px",
        backgroundColor: "rgba(0, 0, 0, 0.78)",
        backdropFilter: "blur(var(--nex-blur-md))",
        WebkitBackdropFilter: "blur(var(--nex-blur-md))",
      }}
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={modalRef}
        style={{
          width: "100%",
          maxWidth,
          backgroundColor: "var(--nex-surface-1)",
          borderRadius: "var(--nex-radius-lg)",
          border: "1px solid var(--nex-border-strong)",
          boxShadow: "var(--nex-shadow-xl)",
          overflow: "hidden",
          animation: "nexFadeInUp 0.25s var(--nex-ease-cinematic)",
        }}
      >
        {title && (
          <div
            style={{
              padding: "16px 20px",
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              borderBottom: "1px solid var(--nex-border-subtle)",
            }}
          >
            <h3 style={{ fontSize: "1.1rem", fontWeight: 600, color: "var(--nex-text-primary)" }}>
              {title}
            </h3>
            <button
              onClick={onClose}
              aria-label="Fechar modal"
              style={{
                background: "transparent",
                border: "none",
                color: "var(--nex-text-secondary)",
                cursor: "pointer",
                padding: "4px",
                display: "inline-flex",
                alignItems: "center",
                justifyContent: "center",
                borderRadius: "var(--nex-radius-sm)",
              }}
            >
              <X size={18} />
            </button>
          </div>
        )}
        <div style={{ padding: "20px", maxHeight: "80vh", overflowY: "auto" }}>
          {children}
        </div>
      </div>
    </div>
  );
};

export interface DrawerProps {
  isOpen: boolean;
  onClose: () => void;
  title?: React.ReactNode;
  children: React.ReactNode;
  position?: "left" | "right";
  width?: string;
}

export const Drawer: React.FC<DrawerProps> = ({
  isOpen,
  onClose,
  title,
  children,
  position = "right",
  width = "320px",
}) => {
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    if (isOpen) {
      document.body.style.overflow = "hidden";
      window.addEventListener("keydown", handleKeyDown);
    }
    return () => {
      document.body.style.overflow = "";
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div
      role="dialog"
      aria-modal="true"
      style={{
        position: "fixed",
        inset: 0,
        zIndex: "var(--nex-z-drawer)",
        backgroundColor: "rgba(0, 0, 0, 0.65)",
        backdropFilter: "blur(var(--nex-blur-sm))",
      }}
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        style={{
          position: "absolute",
          top: 0,
          bottom: 0,
          [position]: 0,
          width: "100%",
          maxWidth: width,
          backgroundColor: "var(--nex-surface-1)",
          borderLeft: position === "right" ? "1px solid var(--nex-border-strong)" : "none",
          borderRight: position === "left" ? "1px solid var(--nex-border-strong)" : "none",
          boxShadow: "var(--nex-shadow-xl)",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            padding: "16px 20px",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            borderBottom: "1px solid var(--nex-border-subtle)",
          }}
        >
          <div style={{ fontWeight: 600, fontSize: "1.05rem" }}>{title}</div>
          <button
            onClick={onClose}
            aria-label="Fechar gaveta"
            style={{
              background: "transparent",
              border: "none",
              color: "var(--nex-text-secondary)",
              cursor: "pointer",
              padding: "4px",
            }}
          >
            <X size={20} />
          </button>
        </div>
        <div style={{ padding: "20px", flex: 1, overflowY: "auto" }}>{children}</div>
      </div>
    </div>
  );
};
