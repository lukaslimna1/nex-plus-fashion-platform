import React from "react";
import { Sparkles, ShieldCheck, BookOpen, Globe, Heart } from "lucide-react";
import { Badge } from "@nex-plus/ui";

export const AboutPage: React.FC = () => {
  return (
    <div style={{ maxWidth: "900px", margin: "0 auto", padding: "32px 24px 80px 24px" }}>
      <div style={{ marginBottom: "32px" }}>
        <Badge variant="tier" style={{ marginBottom: "12px" }}>
          Institucional & Cultural
        </Badge>
        <h1
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "clamp(2.2rem, 5vw, 3.4rem)",
            fontWeight: 800,
            marginBottom: "12px",
            lineHeight: 1.1,
          }}
        >
          Sobre o NEX+ Fashion
        </h1>
        <p style={{ fontSize: "1.1rem", color: "var(--nex-text-secondary)", lineHeight: 1.6 }}>
          Transformando conteúdo disperso e barreiras linguísticas em um arquivo visual canônico, moderno e acessível em português do Brasil.
        </p>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: "28px" }}>
        <div
          style={{
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-subtle)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "28px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "12px" }}>
            <Sparkles size={20} style={{ color: "var(--nex-accent-gold)" }} />
            <h2 style={{ fontSize: "1.2rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
              Propósito e Missão
            </h2>
          </div>
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.6, fontSize: "0.95rem" }}>
            NEX+ Fashion é um acervo gratuito, não oficial, educacional e cultural de Fashion Weeks e conhecimento de moda. Ele foi idealizado para democratizar o acesso ao repertório de moda internacional para estudantes, estilistas, modelistas, costureiros, pesquisadores e entusiastas brasileiros.
          </p>
        </div>

        <div
          style={{
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-subtle)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "28px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "12px" }}>
            <ShieldCheck size={20} style={{ color: "var(--nex-success)" }} />
            <h2 style={{ fontSize: "1.2rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
              Proveniência & Rigor com Mídias
            </h2>
          </div>
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.6, fontSize: "0.95rem", marginBottom: "12px" }}>
            O NEX+ atua como agregador visual inline-first. Isso significa que mídias de terceiros são integradas preservando explicitamente seus criadores, fotógrafos, publicações de origem e links para a página original.
          </p>
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.6, fontSize: "0.95rem" }}>
            Regra fundamental: <strong>Nunca inventar dados para preencher a interface</strong>. Quando uma coleção não possui mídia oficial verificada, o sistema exibe com orgulho o estado de verificação em andamento, respeitando o trabalho dos autores.
          </p>
        </div>

        <div
          style={{
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-subtle)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "28px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "12px" }}>
            <Globe size={20} style={{ color: "#60a5fa" }} />
            <h2 style={{ fontSize: "1.2rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
              Multiplataforma Canônica
            </h2>
          </div>
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.6, fontSize: "0.95rem" }}>
            A aplicação Web é o frontend canônico. A versão PWA oferece recursos de instalação e uso fluido em smartphones. Futuramente, a mesma interface Web alimentará a edição Desktop com capacidades locais exclusivas, como anotações offline no Caderno e reindexação de acervo privado.
          </p>
        </div>
      </div>
    </div>
  );
};
