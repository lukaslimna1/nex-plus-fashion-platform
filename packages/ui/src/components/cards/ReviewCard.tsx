import React from "react";
import { ExternalLink, BookOpen, Quote } from "lucide-react";
import { Badge } from "../primitives/Badge";

export interface ReviewCardProps {
  id: string;
  publication: string;
  criticName?: string;
  title: string;
  excerpt: string;
  maison: string;
  collectionName: string;
  seasonCode: string;
  publishedAt?: string;
  originalUrl?: string;
  onOpenCollectionReview?: () => void;
  className?: string;
}

export const ReviewCard: React.FC<ReviewCardProps> = ({
  publication,
  criticName,
  title,
  excerpt,
  maison,
  collectionName,
  seasonCode,
  publishedAt,
  originalUrl,
  onOpenCollectionReview,
  className = "",
}) => {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
        backgroundColor: "var(--nex-surface-1)",
        borderRadius: "var(--nex-radius-lg)",
        border: "1px solid var(--nex-border-subtle)",
        padding: "20px",
        transition: "all var(--nex-transition-normal)",
        minHeight: "220px",
        width: "100%",
        maxWidth: "360px",
        position: "relative",
      }}
      className={`nex-review-card nex-glass-hover ${className}`}
    >
      <div>
        {/* Header: Publication & Season */}
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            marginBottom: "12px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
            <BookOpen size={14} style={{ color: "var(--nex-accent-gold)" }} />
            <span
              style={{
                fontFamily: "var(--nex-font-sans)",
                fontSize: "0.8rem",
                fontWeight: 700,
                letterSpacing: "0.06em",
                textTransform: "uppercase",
                color: "var(--nex-text-primary)",
              }}
            >
              {publication}
            </span>
          </div>
          <Badge variant="canonical" size="sm">
            {seasonCode}
          </Badge>
        </div>

        {/* Maison & Collection Context */}
        <div
          style={{
            fontSize: "0.78rem",
            color: "var(--nex-accent-gold)",
            fontWeight: 600,
            textTransform: "uppercase",
            letterSpacing: "0.04em",
            marginBottom: "6px",
          }}
        >
          {maison} • {collectionName}
        </div>

        {/* Title */}
        <h4
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "1.05rem",
            fontWeight: 600,
            lineHeight: 1.3,
            color: "var(--nex-text-primary)",
            marginBottom: "10px",
            letterSpacing: "-0.01em",
          }}
        >
          "{title}"
        </h4>

        {/* Excerpt */}
        <p
          style={{
            fontSize: "0.84rem",
            color: "var(--nex-text-secondary)",
            lineHeight: 1.5,
            marginBottom: "14px",
            display: "-webkit-box",
            WebkitLineClamp: 3,
            WebkitBoxOrient: "vertical",
            overflow: "hidden",
          }}
        >
          {excerpt}
        </p>
      </div>

      {/* Footer & Actions */}
      <div
        style={{
          borderTop: "1px solid var(--nex-border-subtle)",
          paddingTop: "12px",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <div style={{ fontSize: "0.74rem", color: "var(--nex-text-muted)" }}>
          {criticName ? `Por ${criticName}` : "Crítica Especializada"}
          {publishedAt && <span> • {publishedAt}</span>}
        </div>

        <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
          {onOpenCollectionReview && (
            <button
              onClick={onOpenCollectionReview}
              style={{
                background: "transparent",
                border: "none",
                color: "var(--nex-text-primary)",
                fontSize: "0.78rem",
                fontWeight: 600,
                cursor: "pointer",
                textDecoration: "underline",
                textUnderlineOffset: "3px",
              }}
            >
              Ler análise
            </button>
          )}

          {originalUrl && (
            <a
              href={originalUrl}
              target="_blank"
              rel="noopener noreferrer"
              title="Ler publicação original"
              style={{
                color: "var(--nex-text-muted)",
                display: "inline-flex",
                alignItems: "center",
                transition: "color var(--nex-transition-fast)",
              }}
              className="nex-source-link-icon"
            >
              <ExternalLink size={14} />
            </a>
          )}
        </div>
      </div>
    </div>
  );
};
