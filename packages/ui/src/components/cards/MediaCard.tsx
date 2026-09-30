import React from "react";
import { Download, ExternalLink, ShieldAlert } from "lucide-react";
import type { DownloadPolicy } from "@nex-plus/types";
import { MediaCredit } from "../media/MediaCredit";

export interface MediaCardProps {
  id: string;
  title?: string;
  thumbnailUrl?: string;
  remoteUrl?: string;
  aspectRatio?: "2/3" | "16/9" | "4/5" | "1/1";
  photographer?: string;
  creator?: string;
  sourceName?: string;
  sourcePageUrl?: string;
  downloadPolicy?: DownloadPolicy;
  isPendingVerification?: boolean;
  onClick?: () => void;
  className?: string;
}

export const MediaCard: React.FC<MediaCardProps> = ({
  title,
  thumbnailUrl,
  remoteUrl,
  aspectRatio = "2/3",
  photographer,
  creator,
  sourceName,
  sourcePageUrl,
  downloadPolicy = "DOWNLOAD_UNKNOWN",
  isPendingVerification = false,
  onClick,
  className = "",
}) => {
  const showDownload = downloadPolicy === "DOWNLOAD_ALLOWED";
  const displayImage = thumbnailUrl || remoteUrl;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        backgroundColor: "var(--nex-surface-1)",
        borderRadius: "var(--nex-radius-md)",
        border: "1px solid var(--nex-border-subtle)",
        overflow: "hidden",
        transition: "all var(--nex-transition-normal)",
      }}
      className={`nex-media-card ${className}`}
    >
      <div
        style={{
          position: "relative",
          width: "100%",
          aspectRatio,
          backgroundColor: "var(--nex-surface-2)",
          cursor: onClick ? "pointer" : "default",
          overflow: "hidden",
        }}
        onClick={onClick}
      >
        {isPendingVerification ? (
          <div
            style={{
              position: "absolute",
              inset: 0,
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              padding: "16px",
              textAlign: "center",
              backgroundColor: "rgba(22, 22, 28, 0.95)",
              color: "var(--nex-text-secondary)",
              gap: "8px",
            }}
          >
            <ShieldAlert size={28} style={{ color: "var(--nex-accent-gold)" }} />
            <div style={{ fontSize: "0.8rem", fontWeight: 600, color: "var(--nex-text-primary)" }}>
              Verificação de Fonte Pendente
            </div>
            <div style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)", maxWidth: "220px" }}>
              Nenhuma mídia não autorizada foi incorporada.
            </div>
          </div>
        ) : displayImage ? (
          <img
            src={displayImage}
            alt={title || "Mídia de moda"}
            loading="lazy"
            style={{
              width: "100%",
              height: "100%",
              objectFit: "cover",
              transition: "transform var(--nex-transition-cinematic)",
            }}
            className="nex-card-image"
          />
        ) : (
          <div
            style={{
              position: "absolute",
              inset: 0,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              color: "var(--nex-text-muted)",
              fontSize: "0.8rem",
            }}
          >
            Sem imagem disponível
          </div>
        )}

        {/* Floating actions if allowed */}
        {showDownload && (
          <div
            style={{
              position: "absolute",
              top: "8px",
              right: "8px",
              zIndex: 2,
            }}
          >
            <button
              title="Baixar imagem (Permitido)"
              aria-label="Baixar imagem"
              style={{
                background: "rgba(0, 0, 0, 0.7)",
                backdropFilter: "blur(4px)",
                border: "1px solid var(--nex-border-subtle)",
                borderRadius: "var(--nex-radius-full)",
                color: "#fff",
                padding: "6px",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
              onClick={(e) => {
                e.stopPropagation();
                if (remoteUrl) window.open(remoteUrl, "_blank");
              }}
            >
              <Download size={14} />
            </button>
          </div>
        )}
      </div>

      <div style={{ padding: "10px 12px" }}>
        {title && (
          <div
            style={{
              fontWeight: 600,
              fontSize: "0.85rem",
              color: "var(--nex-text-primary)",
              marginBottom: "4px",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {title}
          </div>
        )}

        <MediaCredit
          creator={creator}
          photographer={photographer}
          sourceName={sourceName}
          sourcePageUrl={sourcePageUrl}
        />
      </div>
    </div>
  );
};
