import React, { useState, useEffect } from "react";
import { AppShell } from "@nex-plus/ui";
import { HomePage } from "./pages/HomePage";
import { CollectionPage } from "./pages/CollectionPage";
import { RotaGlobalPage } from "./pages/RotaGlobalPage";
import { DesignSystemQA } from "./pages/DesignSystemQA";
import { AboutPage } from "./pages/AboutPage";
import { FirstVisitIntro } from "./components/FirstVisitIntro";
import type { SearchResultItem } from "@nex-plus/ui";

export const App: React.FC = () => {
  // Verificação inicial se já viu a vinheta de introdução
  const [showIntro, setShowIntro] = useState(() => {
    // Permite forçar abertura da página do design system diretamente via URL
    if (window.location.pathname === "/__design-system" || window.location.hash === "#/__design-system") {
      return false;
    }
    return !localStorage.getItem("nex_intro_seen_v1");
  });

  // Roteador simples e reativo suportando hash e pushState
  const [currentRoute, setCurrentRoute] = useState<string>(() => {
    const hash = window.location.hash.replace("#/", "").replace("#", "");
    const path = window.location.pathname.replace(/^\//, "");
    return hash || path || "home";
  });

  useEffect(() => {
    const handlePopState = () => {
      const hash = window.location.hash.replace("#/", "").replace("#", "");
      const path = window.location.pathname.replace(/^\//, "");
      setCurrentRoute(hash || path || "home");
    };
    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  }, []);

  const handleNavigate = (route: string) => {
    setCurrentRoute(route);
    window.location.hash = `#/${route}`;
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const handleSelectSearchResult = (item: SearchResultItem) => {
    if (item.type === "COLLECTION") {
      handleNavigate("collection");
    } else if (item.type === "CITY" || item.type === "EVENT") {
      handleNavigate("rota");
    } else if (item.type === "TERM") {
      handleNavigate("biblioteca");
    } else {
      handleNavigate("maisons");
    }
  };

  return (
    <>
      {showIntro && <FirstVisitIntro onComplete={() => setShowIntro(false)} />}

      <AppShell
        currentRoute={currentRoute}
        onNavigate={handleNavigate}
        onSelectSearchResult={handleSelectSearchResult}
      >
        {currentRoute === "home" && <HomePage onNavigate={handleNavigate} />}
        {currentRoute === "collection" && <CollectionPage />}
        {currentRoute === "rota" && <RotaGlobalPage onNavigate={handleNavigate} />}
        {currentRoute === "__design-system" && <DesignSystemQA />}
        {currentRoute === "sobre" && <AboutPage />}

        {/* Rotas complementares com fallback visual premium */}
        {currentRoute === "maisons" && (
          <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "40px 24px" }}>
            <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", marginBottom: "8px" }}>
              Catálogo de Maisons
            </h1>
            <p style={{ color: "var(--nex-text-secondary)", marginBottom: "32px" }}>
              Diretório de casas de alta costura, ateliês e direções artísticas do circuito internacional.
            </p>
            <HomePage onNavigate={handleNavigate} />
          </div>
        )}

        {currentRoute === "tendencias" && (
          <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "40px 24px" }}>
            <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", marginBottom: "8px" }}>
              Tendências & Evidências
            </h1>
            <p style={{ color: "var(--nex-text-secondary)", marginBottom: "32px" }}>
              Análise com recorte temporal rigoroso sustentado por looks, coleções e críticas catalogadas.
            </p>
            <HomePage onNavigate={handleNavigate} />
          </div>
        )}

        {currentRoute === "biblioteca" && (
          <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "40px 24px" }}>
            <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", marginBottom: "8px" }}>
              Biblioteca de Moda
            </h1>
            <p style={{ color: "var(--nex-text-secondary)", marginBottom: "32px" }}>
              Acervo de termos técnicos, modelagem, tecidos e história da moda em português do Brasil.
            </p>
            <HomePage onNavigate={handleNavigate} />
          </div>
        )}

        {currentRoute === "favoritos" && (
          <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "40px 24px" }}>
            <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", marginBottom: "8px" }}>
              Meus Favoritos
            </h1>
            <p style={{ color: "var(--nex-text-secondary)", marginBottom: "32px" }}>
              Coleções, desfiles, cidades e termos salvos para consulta rápida.
            </p>
            <CollectionPage />
          </div>
        )}

        {currentRoute === "conta" && (
          <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "40px 24px" }}>
            <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", marginBottom: "8px" }}>
              Conta & Preferências
            </h1>
            <p style={{ color: "var(--nex-text-secondary)", marginBottom: "32px" }}>
              Gerenciamento de perfil, temas e sincronização de anotações do Caderno.
            </p>
            <AboutPage />
          </div>
        )}
      </AppShell>
    </>
  );
};
