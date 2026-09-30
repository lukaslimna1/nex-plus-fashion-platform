import React, { useState } from "react";
import { Navbar } from "../navigation/Navbar";
import { SearchOverlay } from "../search/SearchOverlay";
import type { SearchResultItem } from "../search/SearchOverlay";

export interface AppShellProps {
  children: React.ReactNode;
  currentRoute?: string | undefined;
  onNavigate: (route: string) => void;
  onSelectSearchResult?: ((item: SearchResultItem) => void) | undefined;
  isLoggedIn?: boolean | undefined;
}

export const AppShell: React.FC<AppShellProps> = ({
  children,
  currentRoute = "home",
  onNavigate,
  onSelectSearchResult,
  isLoggedIn = false,
}) => {
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  return (
    <div
      style={{
        minHeight: "100vh",
        display: "flex",
        flexDirection: "column",
        backgroundColor: "var(--nex-bg-primary)",
        color: "var(--nex-text-primary)",
      }}
      className="nex-app-shell"
    >
      <Navbar
        currentRoute={currentRoute}
        onNavigate={onNavigate}
        onOpenSearch={() => setIsSearchOpen(true)}
        isLoggedIn={isLoggedIn}
      />

      <main style={{ flex: 1 }} className="nex-app-main">
        {children}
      </main>

      <SearchOverlay
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
        onSelectResult={onSelectSearchResult}
      />

      {/* Editorial Footer */}
      <footer
        style={{
          borderTop: "1px solid var(--nex-border-subtle)",
          backgroundColor: "var(--nex-bg-secondary)",
          padding: "54px 24px 72px 24px",
          marginTop: "64px",
        }}
        className="nex-footer"
      >
        <div
          style={{
            maxWidth: "1280px",
            margin: "0 auto",
            display: "grid",
            gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))",
            gap: "36px",
          }}
        >
          {/* Brand Column */}
          <div>
            <div style={{ display: "flex", alignItems: "baseline", gap: "4px", marginBottom: "12px" }}>
              <span
                style={{
                  fontFamily: "var(--nex-font-display)",
                  fontSize: "1.3rem",
                  fontWeight: 800,
                  letterSpacing: "0.12em",
                }}
              >
                NEX+
              </span>
              <span
                style={{
                  fontFamily: "var(--nex-font-sans)",
                  fontSize: "0.78rem",
                  fontWeight: 600,
                  letterSpacing: "0.2em",
                  color: "var(--nex-accent-gold)",
                }}
              >
                FASHION
              </span>
            </div>
            <p
              style={{
                fontSize: "0.84rem",
                color: "var(--nex-text-secondary)",
                lineHeight: 1.5,
                maxWidth: "300px",
              }}
            >
              Acervo aberto, não oficial, educacional e cultural de Fashion Weeks e conhecimento de moda. Desenvolvido para transformar repertório disperso em uma experiência visual de alta fidelidade.
            </p>
          </div>

          {/* Navigation Column */}
          <div>
            <h4
              style={{
                fontFamily: "var(--nex-font-sans)",
                fontSize: "0.8rem",
                fontWeight: 700,
                textTransform: "uppercase",
                letterSpacing: "0.08em",
                color: "var(--nex-text-primary)",
                marginBottom: "16px",
              }}
            >
              Navegação
            </h4>
            <div style={{ display: "flex", flexDirection: "column", gap: "10px", fontSize: "0.85rem" }}>
              <button
                onClick={() => onNavigate("home")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Início
              </button>
              <button
                onClick={() => onNavigate("rota")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Rota Global da Moda
              </button>
              <button
                onClick={() => onNavigate("maisons")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Catálogo de Maisons
              </button>
              <button
                onClick={() => onNavigate("tendencias")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Tendências & Evidências
              </button>
              <button
                onClick={() => onNavigate("biblioteca")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Biblioteca de Moda
              </button>
            </div>
          </div>

          {/* Institutional Column */}
          <div>
            <h4
              style={{
                fontFamily: "var(--nex-font-sans)",
                fontSize: "0.8rem",
                fontWeight: 700,
                textTransform: "uppercase",
                letterSpacing: "0.08em",
                color: "var(--nex-text-primary)",
                marginBottom: "16px",
              }}
            >
              Institucional
            </h4>
            <div style={{ display: "flex", flexDirection: "column", gap: "10px", fontSize: "0.85rem" }}>
              <button
                onClick={() => onNavigate("sobre")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Sobre o NEX+ Fashion
              </button>
              <button
                onClick={() => onNavigate("proveniencia")}
                style={{ background: "none", border: "none", color: "var(--nex-text-secondary)", textAlign: "left", cursor: "pointer" }}
              >
                Matriz de Fontes & Proveniência
              </button>
              <button
                onClick={() => onNavigate("__design-system")}
                style={{ background: "none", border: "none", color: "var(--nex-accent-gold)", textAlign: "left", cursor: "pointer", fontWeight: 600 }}
              >
                QA / Design System V2
              </button>
            </div>
          </div>

          {/* Ethics & Rights Column */}
          <div>
            <h4
              style={{
                fontFamily: "var(--nex-font-sans)",
                fontSize: "0.8rem",
                fontWeight: 700,
                textTransform: "uppercase",
                letterSpacing: "0.08em",
                color: "var(--nex-text-primary)",
                marginBottom: "16px",
              }}
            >
              Direitos & Agregação
            </h4>
            <p
              style={{
                fontSize: "0.78rem",
                color: "var(--nex-text-muted)",
                lineHeight: 1.5,
              }}
            >
              O NEX+ Fashion é um agregador visual inline-first. Mídias de terceiros são incorporadas com preservação de fotógrafos, criadores e veículos originais. Nenhuma mídia externa é apresentada como propriedade do NEX+.
            </p>
          </div>
        </div>

        <div
          style={{
            maxWidth: "1280px",
            margin: "40px auto 0 auto",
            paddingTop: "24px",
            borderTop: "1px solid var(--nex-border-subtle)",
            display: "flex",
            flexWrap: "wrap",
            justifyContent: "space-between",
            alignItems: "center",
            fontSize: "0.76rem",
            color: "var(--nex-text-muted)",
          }}
        >
          <div>© {new Date().getFullYear()} NEX+ Fashion. Todos os direitos de terceiros reservados aos respectivos autores.</div>
          <div>Português do Brasil • Versão 2.0 Web Canônica</div>
        </div>
      </footer>
    </div>
  );
};
