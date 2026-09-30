import React from "react";
import { ArrowRight, BookMarked } from "lucide-react";
import { Tag } from "../primitives/Chip";

export interface TermCardProps {
  id: string;
  term: string;
  originalLanguageTerm?: string;
  language?: string;
  definition: string;
  category?: string;
  onClick?: () => void;
  className?: string;
}

export const TermCard: React.FC<TermCardProps> = ({
  term,
  originalLanguageTerm,
  language,
  definition,
  category,
  onClick,
  className = "",
}) => {
  return (
    <div
      onClick={onClick}
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
        backgroundColor: "var(--nex-surface-1)",
        borderRadius: "var(--nex-radius-md)",
        border: "1px solid var(--nex-border-subtle)",
        padding: "16px 18px",
        cursor: "pointer",
        transition: "all var(--nex-transition-normal)",
        minHeight: "160px",
        width: "100%",
        maxWidth: "320px",
      }}
      className={`nex-term-card nex-glass-hover ${className}`}
    >
      <div>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            marginBottom: "8px",
          }}
        >
          {category && <Tag variant="gold">{category}</Tag>}
          {language && (
            <span style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)", textTransform: "uppercase" }}>
              {language}
            </span>
          )}
        </div>

        <h4
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "1.15rem",
            fontWeight: 700,
            color: "var(--nex-text-primary)",
            marginBottom: "2px",
            letterSpacing: "0.02em",
          }}
        >
          {term}
        </h4>

        {originalLanguageTerm && (
          <div
            style={{
              fontSize: "0.78rem",
              fontStyle: "italic",
              color: "var(--nex-text-muted)",
              marginBottom: "8px",
            }}
          >
            {originalLanguageTerm}
          </div>
        )}

        <p
          style={{
            fontSize: "0.82rem",
            color: "var(--nex-text-secondary)",
            lineHeight: 1.45,
            display: "-webkit-box",
            WebkitLineClamp: 3,
            WebkitBoxOrient: "vertical",
            overflow: "hidden",
          }}
        >
          {definition}
        </p>
      </div>

      <div
        style={{
          marginTop: "12px",
          display: "flex",
          alignItems: "center",
          gap: "4px",
          fontSize: "0.75rem",
          fontWeight: 600,
          color: "var(--nex-accent)",
        }}
      >
        <span>Ver na Biblioteca</span>
        <ArrowRight size={13} />
      </div>
    </div>
  );
};
