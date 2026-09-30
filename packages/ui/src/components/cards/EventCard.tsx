import React from "react";
import { CalendarDays, Building2 } from "lucide-react";
import { Badge } from "../primitives/Badge";

export interface EventCardProps {
  id: string;
  name: string;
  cityName: string;
  kind?: string;
  organizer?: string;
  coverImageUrl?: string;
  status?: "ACTIVE" | "HISTORICAL" | "VERIFIED";
  onClick?: () => void;
  className?: string;
}

export const EventCard: React.FC<EventCardProps> = ({
  name,
  cityName,
  kind = "Fashion Week",
  organizer,
  coverImageUrl,
  status = "ACTIVE",
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
      className={`nex-event-card nex-glass-hover ${className}`}
    >
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
          className="nex-event-cover"
        />
      ) : (
        <div
          style={{
            position: "absolute",
            inset: 0,
            background: "linear-gradient(135deg, #22222d 0%, #0d0d12 100%)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            padding: "20px",
            textAlign: "center",
          }}
        >
          <CalendarDays size={36} style={{ color: "var(--nex-border-strong)" }} />
        </div>
      )}

      {/* Gradient Overlay */}
      <div
        style={{
          position: "absolute",
          inset: 0,
          background:
            "linear-gradient(180deg, rgba(7, 7, 9, 0.2) 0%, rgba(7, 7, 9, 0.45) 45%, rgba(7, 7, 9, 0.95) 100%)",
        }}
      />

      {/* Badges */}
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
          {kind}
        </Badge>
        {status === "ACTIVE" && (
          <Badge variant="verified" size="sm">
            Ativo
          </Badge>
        )}
      </div>

      {/* Content */}
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
            fontSize: "0.75rem",
            color: "var(--nex-accent-gold)",
            fontWeight: 600,
            textTransform: "uppercase",
            letterSpacing: "0.06em",
            marginBottom: "3px",
          }}
        >
          {cityName}
        </div>

        <h3
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "1.25rem",
            fontWeight: 700,
            color: "var(--nex-text-primary)",
            lineHeight: 1.15,
            marginBottom: "6px",
            letterSpacing: "0.01em",
          }}
        >
          {name}
        </h3>

        {organizer && (
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: "4px",
              fontSize: "0.74rem",
              color: "var(--nex-text-muted)",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            <Building2 size={12} />
            <span>{organizer}</span>
          </div>
        )}
      </div>
    </div>
  );
};
