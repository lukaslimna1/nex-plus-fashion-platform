import React from "react";

export interface MaisonCardProps {
  id: string;
  name: string;
  foundedYear?: number;
  artisticDirection?: string;
  logoUrl?: string;
  coverUrl?: string;
  collectionsCount?: number;
  onClick?: () => void;
  className?: string;
}

export const MaisonCard: React.FC<MaisonCardProps> = ({
  name,
  foundedYear,
  artisticDirection,
  logoUrl,
  collectionsCount,
  onClick,
  className = "",
}) => {
  // Monograma ou iniciais da Maison como fallback elegante de luxo
  const initials = name
    .split(" ")
    .map((n) => n[0])
    .slice(0, 2)
    .join("")
    .toUpperCase();

  return (
    <div
      onClick={onClick}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        textAlign: "center",
        padding: "16px 12px",
        backgroundColor: "var(--nex-surface-1)",
        borderRadius: "var(--nex-radius-lg)",
        border: "1px solid var(--nex-border-subtle)",
        cursor: "pointer",
        transition: "all var(--nex-transition-normal)",
        userSelect: "none",
        width: "100%",
        maxWidth: "200px",
      }}
      className={`nex-maison-card nex-glass-hover ${className}`}
    >
      {/* Circular Emblem / Logo Container */}
      <div
        style={{
          width: "90px",
          height: "90px",
          borderRadius: "50%",
          backgroundColor: "var(--nex-surface-2)",
          border: "2px solid var(--nex-border-strong)",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          overflow: "hidden",
          marginBottom: "12px",
          boxShadow: "var(--nex-shadow-sm)",
          transition: "transform var(--nex-transition-normal), border-color var(--nex-transition-normal)",
        }}
        className="nex-maison-avatar"
      >
        {logoUrl ? (
          <img
            src={logoUrl}
            alt={name}
            style={{ width: "100%", height: "100%", objectFit: "contain", padding: "10px" }}
          />
        ) : (
          <span
            style={{
              fontFamily: "var(--nex-font-display)",
              fontSize: "1.4rem",
              fontWeight: 700,
              letterSpacing: "0.05em",
              color: "var(--nex-text-primary)",
            }}
          >
            {initials}
          </span>
        )}
      </div>

      {/* Maison Details */}
      <h4
        style={{
          fontFamily: "var(--nex-font-sans)",
          fontSize: "0.95rem",
          fontWeight: 600,
          color: "var(--nex-text-primary)",
          marginBottom: "4px",
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          width: "100%",
        }}
      >
        {name}
      </h4>

      {artisticDirection && (
        <div
          style={{
            fontSize: "0.75rem",
            color: "var(--nex-text-muted)",
            marginBottom: "4px",
            whiteSpace: "nowrap",
            overflow: "hidden",
            textOverflow: "ellipsis",
            width: "100%",
          }}
        >
          {artisticDirection}
        </div>
      )}

      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "6px",
          fontSize: "0.72rem",
          color: "var(--nex-accent-gold)",
          fontWeight: 600,
          textTransform: "uppercase",
          letterSpacing: "0.04em",
        }}
      >
        {foundedYear ? `Desde ${foundedYear}` : "Maison Oficial"}
        {collectionsCount !== undefined && <span>• {collectionsCount} coleções</span>}
      </div>
    </div>
  );
};
