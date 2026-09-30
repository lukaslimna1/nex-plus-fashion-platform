import React, { useState } from "react";
import { Sparkles, Calendar, Layers } from "lucide-react";
import { Badge } from "../primitives/Badge";

export interface CollectionCardProps {
  id: string;
  name: string;
  maison: string;
  seasonLabel: string;
  seasonCode: string;
  presentedOn?: string | undefined;
  format?: "SHOW" | "PRESENTATION" | "FILM" | "OTHER" | undefined;
  coverImageUrl?: string | undefined;
  isPendingVerification?: boolean | undefined;
  onClick?: (() => void) | undefined;
  className?: string | undefined;
}

export const CollectionCard: React.FC<CollectionCardProps> = ({
  name,
  maison,
  seasonLabel,
  seasonCode,
  presentedOn,
  format = "SHOW",
  coverImageUrl,
  isPendingVerification = false,
  onClick,
  className = "",
}) => {
  const [isHovered, setIsHovered] = useState(false);

  const formatLabels: Record<string, string> = {
    SHOW: "Desfile",
    PRESENTATION: "Apresentação",
    FILM: "Filme Digital",
    OTHER: "Especial",
  };

  return (
    <div
      style={{
        position: "relative",
        width: "100%",
        borderRadius: "var(--nex-radius-md)",
        backgroundColor: "var(--nex-surface-1)",
        border: "1px solid var(--nex-border-subtle)",
        overflow: "visible",
        cursor: "pointer",
        transition: "border-color var(--nex-transition-normal)",
      }}
      className={`nex-collection-card ${className}`}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onClick={onClick}
    >
      {/* Base Card */}
      <div
        style={{
          width: "100%",
          borderRadius: "var(--nex-radius-md)",
          overflow: "hidden",
          backgroundColor: "var(--nex-surface-1)",
        }}
      >
        <div
          style={{
            position: "relative",
            width: "100%",
            aspectRatio: "3/4",
            backgroundColor: "var(--nex-surface-2)",
            overflow: "hidden",
          }}
        >
          {isPendingVerification || !coverImageUrl ? (
            <div
              style={{
                width: "100%",
                height: "100%",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
                padding: "20px",
                textAlign: "center",
                background: "linear-gradient(135deg, #171720 0%, #0d0d12 100%)",
              }}
            >
              <span
                style={{
                  fontFamily: "var(--nex-font-display)",
                  fontSize: "1.6rem",
                  letterSpacing: "0.05em",
                  color: "var(--nex-text-primary)",
                  marginBottom: "8px",
                  textTransform: "uppercase",
                }}
              >
                {maison}
              </span>
              <span
                style={{
                  fontSize: "0.75rem",
                  color: "var(--nex-accent-gold)",
                  letterSpacing: "0.08em",
                  textTransform: "uppercase",
                  fontWeight: 600,
                }}
              >
                {seasonCode}
              </span>
            </div>
          ) : (
            <img
              src={coverImageUrl}
              alt={name}
              loading="lazy"
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
                transform: isHovered ? "scale(1.04)" : "scale(1)",
                transition: "transform var(--nex-transition-smooth)",
              }}
            />
          )}

          {/* Badges on top */}
          <div
            style={{
              position: "absolute",
              top: "10px",
              left: "10px",
              display: "flex",
              gap: "6px",
              zIndex: 2,
            }}
          >
            <Badge variant="canonical" size="sm">
              {seasonCode}
            </Badge>
            {format && (
              <Badge variant="neutral" size="sm">
                {formatLabels[format] || format}
              </Badge>
            )}
          </div>
        </div>

        {/* Card Body */}
        <div style={{ padding: "12px 14px" }}>
          <div
            style={{
              fontFamily: "var(--nex-font-sans)",
              fontSize: "0.75rem",
              textTransform: "uppercase",
              letterSpacing: "0.06em",
              color: "var(--nex-text-muted)",
              marginBottom: "3px",
              fontWeight: 600,
            }}
          >
            {maison}
          </div>
          <h4
            style={{
              fontSize: "0.95rem",
              fontWeight: 600,
              color: "var(--nex-text-primary)",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
              marginBottom: "6px",
            }}
          >
            {seasonLabel}
          </h4>

          {presentedOn && (
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: "5px",
                fontSize: "0.75rem",
                color: "var(--nex-text-secondary)",
              }}
            >
              <Calendar size={12} style={{ color: "var(--nex-text-muted)" }} />
              <span>Apresentado em {presentedOn}</span>
            </div>
          )}
        </div>
      </div>

      {/* Streaming Expanded Hover Card (Desktop overlay) */}
      {isHovered && (
        <div
          style={{
            position: "absolute",
            top: "-12px",
            left: "-12px",
            right: "-12px",
            zIndex: "var(--nex-z-card-hover)",
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-strong)",
            borderRadius: "var(--nex-radius-lg)",
            boxShadow: "var(--nex-shadow-xl)",
            padding: "16px",
            pointerEvents: "auto",
            animation: "nexFadeIn 0.2s var(--nex-ease-cinematic)",
            display: "none", // via media query no desktop
          }}
          className="nex-hover-overlay"
        >
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", marginBottom: "8px" }}>
            <div>
              <div style={{ fontSize: "0.75rem", color: "var(--nex-accent-gold)", fontWeight: 700, textTransform: "uppercase" }}>
                {maison}
              </div>
              <div style={{ fontSize: "1rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
                {seasonLabel}
              </div>
            </div>
            <Badge variant="canonical" size="sm">
              {seasonCode}
            </Badge>
          </div>

          <div style={{ display: "flex", gap: "10px", fontSize: "0.75rem", color: "var(--nex-text-secondary)", marginBottom: "12px" }}>
            <span>Formato: {formatLabels[format] || format}</span>
            {presentedOn && <span>• {presentedOn}</span>}
          </div>

          <button
            style={{
              width: "100%",
              padding: "8px",
              backgroundColor: "var(--nex-accent)",
              color: "#000",
              border: "none",
              borderRadius: "var(--nex-radius-sm)",
              fontWeight: 600,
              fontSize: "0.85rem",
              cursor: "pointer",
            }}
          >
            Explorar Coleção
          </button>
        </div>
      )}
    </div>
  );
};
