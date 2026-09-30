import React from "react";
import { ExternalLink, Camera, BookOpen } from "lucide-react";

export interface MediaCreditProps {
  creator?: string | undefined;
  photographer?: string | undefined;
  creditLine?: string | undefined;
  sourceName?: string | undefined;
  sourcePageUrl?: string | undefined;
  className?: string | undefined;
  style?: React.CSSProperties | undefined;
}

export const MediaCredit: React.FC<MediaCreditProps> = ({
  creator,
  photographer,
  creditLine,
  sourceName,
  sourcePageUrl,
  className = "",
  style,
}) => {
  const photoCredit = photographer || creator;

  return (
    <div
      style={{
        display: "flex",
        flexWrap: "wrap",
        alignItems: "center",
        gap: "10px",
        fontSize: "0.78rem",
        color: "var(--nex-text-secondary)",
        padding: "6px 0",
        ...style,
      }}
      className={`nex-media-credit ${className}`}
    >
      {photoCredit && (
        <span style={{ display: "inline-flex", alignItems: "center", gap: "4px" }}>
          <Camera size={13} style={{ color: "var(--nex-text-muted)" }} />
          <span>Foto:</span>
          <strong style={{ color: "var(--nex-text-primary)", fontWeight: 500 }}>
            {photoCredit}
          </strong>
        </span>
      )}

      {sourceName && (
        <span style={{ display: "inline-flex", alignItems: "center", gap: "4px" }}>
          <BookOpen size={13} style={{ color: "var(--nex-text-muted)" }} />
          <span>Fonte:</span>
          <strong style={{ color: "var(--nex-text-primary)", fontWeight: 500 }}>
            {sourceName}
          </strong>
        </span>
      )}

      {creditLine && !photoCredit && (
        <span style={{ color: "var(--nex-text-muted)", fontStyle: "italic" }}>
          {creditLine}
        </span>
      )}

      {sourcePageUrl && (
        <a
          href={sourcePageUrl}
          target="_blank"
          rel="noopener noreferrer"
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "3px",
            color: "var(--nex-text-primary)",
            textDecoration: "underline",
            textUnderlineOffset: "3px",
            fontWeight: 500,
            marginLeft: "auto",
          }}
          className="nex-source-link"
        >
          <span>Ver original</span>
          <ExternalLink size={12} />
        </a>
      )}
    </div>
  );
};

export interface SourceLinkProps {
  label: string;
  url: string;
  sourceType?: string;
  authorityTier?: string;
}

export const SourceLink: React.FC<SourceLinkProps> = ({
  label,
  url,
  sourceType,
  authorityTier,
}) => {
  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      style={{
        display: "inline-flex",
        alignItems: "center",
        gap: "6px",
        padding: "6px 12px",
        backgroundColor: "var(--nex-surface-2)",
        borderRadius: "var(--nex-radius-sm)",
        color: "var(--nex-text-primary)",
        fontSize: "0.8rem",
        textDecoration: "none",
        border: "1px solid var(--nex-border-subtle)",
        transition: "border-color var(--nex-transition-fast)",
      }}
      className="nex-source-pill"
    >
      <span>{label}</span>
      {authorityTier && (
        <span
          style={{
            fontSize: "0.65rem",
            padding: "1px 4px",
            borderRadius: "2px",
            backgroundColor: "rgba(212, 175, 55, 0.15)",
            color: "var(--nex-accent-gold)",
            fontWeight: 700,
          }}
        >
          Tier {authorityTier}
        </span>
      )}
      <ExternalLink size={12} style={{ color: "var(--nex-text-muted)" }} />
    </a>
  );
};

export interface MetadataLineProps {
  label: string;
  value: React.ReactNode;
}

export const MetadataLine: React.FC<MetadataLineProps> = ({ label, value }) => {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "baseline",
        padding: "6px 0",
        borderBottom: "1px solid var(--nex-border-subtle)",
        fontSize: "0.85rem",
      }}
    >
      <span style={{ color: "var(--nex-text-muted)", textTransform: "uppercase", fontSize: "0.72rem", letterSpacing: "0.05em" }}>
        {label}
      </span>
      <span style={{ color: "var(--nex-text-primary)", fontWeight: 500, textAlign: "right" }}>
        {value}
      </span>
    </div>
  );
};
