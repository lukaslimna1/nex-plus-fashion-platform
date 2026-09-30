import React, { useState, useEffect } from "react";
import { Sparkles, ArrowRight } from "lucide-react";

export interface FirstVisitIntroProps {
  onComplete: () => void;
}

export const FirstVisitIntro: React.FC<FirstVisitIntroProps> = ({ onComplete }) => {
  const [isDismissing, setIsDismissing] = useState(false);

  useEffect(() => {
    // Se o usuário prefere movimentos reduzidos, completa de imediato
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      handleFinish();
      return;
    }

    // Auto-avanço suave após 4.5 segundos
    const timer = setTimeout(() => {
      handleFinish();
    }, 4500);

    return () => clearTimeout(timer);
  }, []);

  const handleFinish = () => {
    setIsDismissing(true);
    localStorage.setItem("nex_intro_seen_v1", "true");
    setTimeout(() => {
      onComplete();
    }, 400);
  };

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 9999,
        backgroundColor: "#070709",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        padding: "24px",
        textAlign: "center",
        opacity: isDismissing ? 0 : 1,
        transition: "opacity 0.4s cubic-bezier(0.16, 1, 0.3, 1)",
        pointerEvents: isDismissing ? "none" : "auto",
      }}
      className="nex-first-visit-intro"
    >
      <div style={{ maxWidth: "600px", animation: "nexFadeInUp 0.8s ease-out" }}>
        <div
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "6px",
            fontSize: "0.8rem",
            color: "var(--nex-accent-gold)",
            textTransform: "uppercase",
            letterSpacing: "0.2em",
            fontWeight: 700,
            marginBottom: "16px",
          }}
        >
          <Sparkles size={14} />
          <span>Acervo Canônico</span>
        </div>

        <h1
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "clamp(2.5rem, 8vw, 4.5rem)",
            fontWeight: 800,
            letterSpacing: "0.14em",
            color: "#ffffff",
            lineHeight: 1,
            marginBottom: "8px",
          }}
        >
          NEX+ FASHION
        </h1>

        <p
          style={{
            fontFamily: "var(--nex-font-sans)",
            fontSize: "1.05rem",
            color: "var(--nex-text-secondary)",
            letterSpacing: "0.04em",
            marginBottom: "32px",
            lineHeight: 1.5,
          }}
        >
          A nova geração do arquivo audiovisual de moda em língua portuguesa.
        </p>

        <button
          onClick={handleFinish}
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "8px",
            padding: "12px 28px",
            backgroundColor: "#ffffff",
            color: "#000000",
            border: "none",
            borderRadius: "var(--nex-radius-full)",
            fontWeight: 600,
            fontSize: "0.95rem",
            cursor: "pointer",
            boxShadow: "0 0 24px rgba(255, 255, 255, 0.25)",
            transition: "all var(--nex-transition-fast)",
          }}
        >
          <span>Entrar no Acervo</span>
          <ArrowRight size={16} />
        </button>
      </div>

      {/* Skip Button Top Right */}
      <button
        onClick={handleFinish}
        style={{
          position: "absolute",
          top: "24px",
          right: "24px",
          background: "transparent",
          border: "none",
          color: "var(--nex-text-muted)",
          fontSize: "0.85rem",
          cursor: "pointer",
          padding: "8px 12px",
          textDecoration: "underline",
          textUnderlineOffset: "3px",
        }}
      >
        Pular
      </button>
    </div>
  );
};
