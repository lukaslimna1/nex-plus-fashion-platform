import React from "react";
import { AlertCircle, RotateCcw, Sparkles } from "lucide-react";
import { Button } from "../primitives/Button";

export interface EmptyStateProps {
  title: string;
  description: string;
  icon?: React.ReactNode;
  actionText?: string;
  onActionClick?: () => void;
  className?: string;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  description,
  icon,
  actionText,
  onActionClick,
  className = "",
}) => {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        padding: "48px 24px",
        textAlign: "center",
        backgroundColor: "var(--nex-surface-1)",
        borderRadius: "var(--nex-radius-lg)",
        border: "1px dashed var(--nex-border-strong)",
        width: "100%",
      }}
      className={`nex-empty-state ${className}`}
    >
      <div
        style={{
          width: "52px",
          height: "52px",
          borderRadius: "50%",
          backgroundColor: "var(--nex-surface-2)",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          color: "var(--nex-text-secondary)",
          marginBottom: "16px",
        }}
      >
        {icon || <Sparkles size={24} style={{ color: "var(--nex-accent-gold)" }} />}
      </div>

      <h3
        style={{
          fontFamily: "var(--nex-font-sans)",
          fontSize: "1.1rem",
          fontWeight: 600,
          color: "var(--nex-text-primary)",
          marginBottom: "6px",
        }}
      >
        {title}
      </h3>

      <p
        style={{
          fontSize: "0.88rem",
          color: "var(--nex-text-secondary)",
          maxWidth: "420px",
          lineHeight: 1.5,
          marginBottom: actionText ? "20px" : "0",
        }}
      >
        {description}
      </p>

      {actionText && onActionClick && (
        <Button variant="secondary" size="sm" onClick={onActionClick}>
          {actionText}
        </Button>
      )}
    </div>
  );
};

export interface ErrorStateProps {
  title?: string;
  message: string;
  onRetry?: () => void;
  className?: string;
}

export const ErrorState: React.FC<ErrorStateProps> = ({
  title = "Falha ao carregar conteúdo",
  message,
  onRetry,
  className = "",
}) => {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        padding: "40px 20px",
        textAlign: "center",
        backgroundColor: "rgba(244, 63, 94, 0.06)",
        borderRadius: "var(--nex-radius-lg)",
        border: "1px solid rgba(244, 63, 94, 0.2)",
        width: "100%",
      }}
      className={`nex-error-state ${className}`}
    >
      <AlertCircle size={36} style={{ color: "var(--nex-error)", marginBottom: "12px" }} />
      <h3
        style={{
          fontSize: "1.05rem",
          fontWeight: 600,
          color: "var(--nex-text-primary)",
          marginBottom: "6px",
        }}
      >
        {title}
      </h3>
      <p
        style={{
          fontSize: "0.85rem",
          color: "var(--nex-text-secondary)",
          maxWidth: "400px",
          lineHeight: 1.45,
          marginBottom: onRetry ? "18px" : "0",
        }}
      >
        {message}
      </p>
      {onRetry && (
        <Button
          variant="outline"
          size="sm"
          iconLeft={<RotateCcw size={14} />}
          onClick={onRetry}
        >
          Tentar novamente
        </Button>
      )}
    </div>
  );
};

export interface SkeletonProps {
  width?: string | number;
  height?: string | number;
  borderRadius?: string;
  aspectRatio?: string;
  className?: string;
  style?: React.CSSProperties;
}

export const Skeleton: React.FC<SkeletonProps> = ({
  width = "100%",
  height,
  borderRadius = "var(--nex-radius-md)",
  aspectRatio,
  className = "",
  style,
}) => {
  return (
    <div
      style={{
        width,
        height,
        aspectRatio,
        borderRadius,
        backgroundColor: "var(--nex-surface-2)",
        backgroundImage:
          "linear-gradient(90deg, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.04) 50%, rgba(255, 255, 255, 0) 100%)",
        backgroundSize: "200% 100%",
        animation: "nexShimmer 1.8s infinite ease-in-out",
        ...style,
      }}
      className={`nex-skeleton ${className}`}
    />
  );
};
