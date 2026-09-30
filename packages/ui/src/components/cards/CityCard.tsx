import React from "react";
import { MapPin, Globe } from "lucide-react";
import { Badge } from "../primitives/Badge";

export interface CityCardProps {
  id: string;
  name: string;
  country: string;
  region: string;
  coverImageUrl?: string;
  eventsCount?: number;
  hubImportance?: "Global" | "Nacional" | "Regional" | "Especializada";
  onClick?: () => void;
  className?: string;
}

export const CityCard: React.FC<CityCardProps> = ({
  name,
  country,
  region,
  coverImageUrl,
  eventsCount,
  hubImportance,
  onClick,
  className = "",
}) => {
  return (
    <div
      onClick={onClick}
      style={{
        position: "relative",
        width: "100%",
        aspectRatio: "2/3",
        borderRadius: "var(--nex-radius-lg)",
        overflow: "hidden",
        backgroundColor: "var(--nex-surface-1)",
        border: "1px solid var(--nex-border-subtle)",
        cursor: "pointer",
        transition: "all var(--nex-transition-normal)",
      }}
      className={`nex-city-card nex-glass-hover ${className}`}
    >
      {/* City Cover Image */}
      {coverImageUrl ? (
        <img
          src={coverImageUrl}
          alt={name}
          loading="lazy"
          style={{
            position: "absolute",
            inset: 0,
            width: "100%",
            height: "100%",
            objectFit: "cover",
            transition: "transform var(--nex-transition-cinematic)",
          }}
          className="nex-city-cover"
        />
      ) : (
        <div
          style={{
            position: "absolute",
            inset: 0,
            background: "linear-gradient(180deg, #1b1b24 0%, #0d0d12 100%)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <Globe size={40} style={{ color: "var(--nex-border-strong)" }} />
        </div>
      )}

      {/* Dark gradient for editorial legibility */}
      <div
        style={{
          position: "absolute",
          inset: 0,
          background:
            "linear-gradient(180deg, rgba(7, 7, 9, 0.2) 0%, rgba(7, 7, 9, 0.4) 40%, rgba(7, 7, 9, 0.95) 100%)",
        }}
      />

      {/* Top Badges */}
      <div
        style={{
          position: "absolute",
          top: "12px",
          left: "12px",
          right: "12px",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          zIndex: 2,
        }}
      >
        <Badge variant="canonical" size="sm">
          {region}
        </Badge>
        {hubImportance === "Global" && (
          <Badge variant="tier" size="sm">
            Global Hub
          </Badge>
        )}
      </div>

      {/* Bottom Content */}
      <div
        style={{
          position: "absolute",
          bottom: 0,
          left: 0,
          right: 0,
          padding: "16px",
          zIndex: 2,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "4px",
            fontSize: "0.75rem",
            color: "var(--nex-accent-gold)",
            fontWeight: 600,
            textTransform: "uppercase",
            letterSpacing: "0.06em",
            marginBottom: "3px",
          }}
        >
          <MapPin size={12} />
          <span>{country}</span>
        </div>

        <h3
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "1.45rem",
            fontWeight: 700,
            color: "var(--nex-text-primary)",
            lineHeight: 1.1,
            marginBottom: "4px",
            letterSpacing: "0.02em",
          }}
        >
          {name}
        </h3>

        {eventsCount !== undefined && (
          <div style={{ fontSize: "0.78rem", color: "var(--nex-text-secondary)" }}>
            {eventsCount} {eventsCount === 1 ? "evento de moda" : "eventos de moda"}
          </div>
        )}
      </div>
    </div>
  );
};
