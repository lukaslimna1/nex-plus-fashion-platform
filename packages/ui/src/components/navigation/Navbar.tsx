import React, { useState } from "react";
import { Search, Bookmark, Menu, X, Compass, Sparkles, Building2, BookMarked, Home, User } from "lucide-react";
import { AccountMenu } from "./AccountMenu";
import { Drawer } from "../primitives/Modal";

export type NavRoute = "home" | "rota" | "maisons" | "tendencias" | "biblioteca" | "favoritos" | "conta" | "collection";

export interface NavbarProps {
  currentRoute?: string;
  onNavigate: (route: string) => void;
  onOpenSearch: () => void;
  isLoggedIn?: boolean;
}

export const Navbar: React.FC<NavbarProps> = ({
  currentRoute = "home",
  onNavigate,
  onOpenSearch,
  isLoggedIn = false,
}) => {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const navLinks = [
    { id: "rota", label: "Rota Global" },
    { id: "maisons", label: "Maisons" },
    { id: "tendencias", label: "Tendências" },
    { id: "biblioteca", label: "Biblioteca" },
  ];

  return (
    <>
      <header
        style={{
          position: "sticky",
          top: 0,
          zIndex: "var(--nex-z-header)",
          width: "100%",
          height: "64px",
          backgroundColor: "rgba(7, 7, 9, 0.85)",
          backdropFilter: "blur(var(--nex-blur-lg))",
          WebkitBackdropFilter: "blur(var(--nex-blur-lg))",
          borderBottom: "1px solid var(--nex-border-subtle)",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "0 24px",
          transition: "background-color var(--nex-transition-normal)",
        }}
        className="nex-navbar"
      >
        {/* Left: Brand Logo */}
        <div style={{ display: "flex", alignItems: "center", gap: "32px" }}>
          <div
            onClick={() => onNavigate("home")}
            style={{
              display: "flex",
              alignItems: "baseline",
              gap: "4px",
              cursor: "pointer",
              userSelect: "none",
            }}
            className="nex-brand-logo"
          >
            <span
              style={{
                fontFamily: "var(--nex-font-display)",
                fontSize: "1.3rem",
                fontWeight: 800,
                letterSpacing: "0.14em",
                color: "var(--nex-text-primary)",
              }}
            >
              NEX+
            </span>
            <span
              style={{
                fontFamily: "var(--nex-font-sans)",
                fontSize: "0.78rem",
                fontWeight: 600,
                letterSpacing: "0.22em",
                color: "var(--nex-accent-gold)",
                textTransform: "uppercase",
              }}
            >
              FASHION
            </span>
          </div>

          {/* Desktop Center Navigation */}
          <nav
            style={{
              display: "flex",
              alignItems: "center",
              gap: "24px",
            }}
            className="nex-nav-desktop"
          >
            {navLinks.map((link) => {
              const isActive = currentRoute === link.id;
              return (
                <button
                  key={link.id}
                  onClick={() => onNavigate(link.id)}
                  style={{
                    background: "transparent",
                    border: "none",
                    color: isActive ? "var(--nex-accent)" : "var(--nex-text-secondary)",
                    fontSize: "0.88rem",
                    fontWeight: isActive ? 600 : 500,
                    cursor: "pointer",
                    padding: "6px 0",
                    position: "relative",
                    transition: "color var(--nex-transition-fast)",
                    letterSpacing: "0.01em",
                  }}
                  className={`nex-nav-link ${isActive ? "nex-nav-link-active" : ""}`}
                >
                  {link.label}
                  {isActive && (
                    <span
                      style={{
                        position: "absolute",
                        bottom: 0,
                        left: 0,
                        right: 0,
                        height: "2px",
                        backgroundColor: "var(--nex-accent)",
                        borderRadius: "2px",
                      }}
                    />
                  )}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Right Actions */}
        <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
          {/* Global Search Button */}
          <button
            onClick={onOpenSearch}
            aria-label="Abrir busca global"
            style={{
              display: "flex",
              alignItems: "center",
              gap: "8px",
              padding: "6px 12px",
              backgroundColor: "var(--nex-surface-2)",
              border: "1px solid var(--nex-border-subtle)",
              borderRadius: "var(--nex-radius-full)",
              color: "var(--nex-text-secondary)",
              fontSize: "0.82rem",
              cursor: "pointer",
              transition: "all var(--nex-transition-fast)",
            }}
            className="nex-search-trigger"
          >
            <Search size={15} />
            <span className="nex-search-placeholder">Buscar</span>
            <kbd
              style={{
                fontSize: "0.68rem",
                color: "var(--nex-text-muted)",
                backgroundColor: "var(--nex-surface-1)",
                padding: "1px 5px",
                borderRadius: "3px",
                border: "1px solid var(--nex-border-strong)",
              }}
              className="nex-search-kbd"
            >
              ⌘K
            </kbd>
          </button>

          {/* Favorites Button */}
          <button
            onClick={() => onNavigate("favoritos")}
            aria-label="Favoritos"
            style={{
              background: "transparent",
              border: "none",
              color: currentRoute === "favoritos" ? "var(--nex-accent)" : "var(--nex-text-secondary)",
              cursor: "pointer",
              padding: "8px",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              borderRadius: "50%",
              transition: "color var(--nex-transition-fast)",
            }}
            className="nex-favorites-btn"
          >
            <Bookmark size={18} />
          </button>

          {/* User Account Dropdown */}
          <AccountMenu
            isLoggedIn={isLoggedIn}
            onOpenAbout={() => onNavigate("sobre")}
            onOpenFavorites={() => onNavigate("favoritos")}
            onOpenSettings={() => onNavigate("conta")}
          />

          {/* Mobile Hamburger Button */}
          <button
            onClick={() => setIsMobileMenuOpen(true)}
            aria-label="Menu"
            style={{
              background: "transparent",
              border: "none",
              color: "var(--nex-text-primary)",
              cursor: "pointer",
              padding: "6px",
            }}
            className="nex-hamburger-btn"
          >
            <Menu size={22} />
          </button>
        </div>
      </header>

      {/* Mobile Drawer Navigation */}
      <Drawer
        isOpen={isMobileMenuOpen}
        onClose={() => setIsMobileMenuOpen(false)}
        title="Navegação NEX+"
        position="right"
      >
        <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
          {[
            { id: "home", label: "Início", icon: <Home size={18} /> },
            { id: "rota", label: "Rota Global", icon: <Compass size={18} /> },
            { id: "maisons", label: "Maisons", icon: <Building2 size={18} /> },
            { id: "tendencias", label: "Tendências", icon: <Sparkles size={18} /> },
            { id: "biblioteca", label: "Biblioteca", icon: <BookMarked size={18} /> },
            { id: "favoritos", label: "Favoritos", icon: <Bookmark size={18} /> },
            { id: "conta", label: "Conta / Perfil", icon: <User size={18} /> },
          ].map((item) => (
            <button
              key={item.id}
              onClick={() => {
                onNavigate(item.id);
                setIsMobileMenuOpen(false);
              }}
              style={{
                display: "flex",
                alignItems: "center",
                gap: "12px",
                padding: "12px 16px",
                borderRadius: "var(--nex-radius-md)",
                backgroundColor: currentRoute === item.id ? "var(--nex-surface-2)" : "transparent",
                color: currentRoute === item.id ? "var(--nex-accent)" : "var(--nex-text-primary)",
                border: "none",
                fontSize: "0.95rem",
                fontWeight: currentRoute === item.id ? 600 : 500,
                textAlign: "left",
                cursor: "pointer",
              }}
            >
              {item.icon}
              <span>{item.label}</span>
            </button>
          ))}
        </div>
      </Drawer>

      {/* Mobile Bottom Navigation Bar (PWA Friendly) */}
      <nav
        style={{
          position: "fixed",
          bottom: 0,
          left: 0,
          right: 0,
          height: "60px",
          backgroundColor: "rgba(7, 7, 9, 0.92)",
          backdropFilter: "blur(var(--nex-blur-lg))",
          WebkitBackdropFilter: "blur(var(--nex-blur-lg))",
          borderTop: "1px solid var(--nex-border-subtle)",
          zIndex: "var(--nex-z-header)",
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
          padding: "0 8px",
        }}
        className="nex-bottom-nav"
      >
        {[
          { id: "home", label: "Início", icon: <Home size={20} /> },
          { id: "rota", label: "Rota", icon: <Compass size={20} /> },
          { id: "search", label: "Buscar", icon: <Search size={20} />, isAction: true },
          { id: "maisons", label: "Maisons", icon: <Building2 size={20} /> },
          { id: "conta", label: "Perfil", icon: <User size={20} /> },
        ].map((item) => {
          const isActive = currentRoute === item.id;
          return (
            <button
              key={item.id}
              onClick={() => {
                if (item.isAction) {
                  onOpenSearch();
                } else {
                  onNavigate(item.id);
                }
              }}
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                gap: "3px",
                background: "transparent",
                border: "none",
                color: isActive ? "var(--nex-accent)" : "var(--nex-text-muted)",
                fontSize: "0.68rem",
                fontWeight: isActive ? 600 : 500,
                cursor: "pointer",
                padding: "6px 12px",
              }}
            >
              {item.icon}
              <span>{item.label}</span>
            </button>
          );
        })}
      </nav>
    </>
  );
};
