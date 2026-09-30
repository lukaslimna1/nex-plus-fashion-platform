import React, { useState, useEffect, useRef } from "react";
import { Search, X, MapPin, Calendar, Sparkles, Building2, BookMarked, ArrowUpRight } from "lucide-react";

export interface SearchResultItem {
  id: string;
  type: "CITY" | "EVENT" | "COLLECTION" | "MAISON" | "TERM";
  title: string;
  subtitle?: string;
  badge?: string;
  url?: string;
}

export interface SearchInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  onSearch?: (query: string) => void;
  showShortcut?: boolean;
}

export const SearchInput: React.FC<SearchInputProps> = ({
  placeholder = "Buscar coleções, maisons, cidades...",
  onSearch,
  showShortcut = true,
  className = "",
  style,
  ...props
}) => {
  return (
    <div
      style={{
        position: "relative",
        display: "flex",
        alignItems: "center",
        width: "100%",
        maxWidth: "340px",
        ...style,
      }}
      className={`nex-search-input-wrapper ${className}`}
    >
      <Search
        size={16}
        style={{
          position: "absolute",
          left: "12px",
          color: "var(--nex-text-muted)",
          pointerEvents: "none",
        }}
      />
      <input
        type="search"
        placeholder={placeholder}
        style={{
          width: "100%",
          padding: "8px 40px 8px 36px",
          backgroundColor: "var(--nex-surface-2)",
          border: "1px solid var(--nex-border-subtle)",
          borderRadius: "var(--nex-radius-full)",
          color: "var(--nex-text-primary)",
          fontSize: "0.85rem",
          outline: "none",
          transition: "all var(--nex-transition-fast)",
        }}
        {...props}
      />
      {showShortcut && (
        <span
          style={{
            position: "absolute",
            right: "10px",
            fontSize: "0.68rem",
            color: "var(--nex-text-muted)",
            border: "1px solid var(--nex-border-strong)",
            borderRadius: "4px",
            padding: "1px 5px",
            pointerEvents: "none",
          }}
        >
          ⌘K
        </span>
      )}
    </div>
  );
};

export interface SearchOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectResult?: ((item: SearchResultItem) => void) | undefined;
}

export const SearchOverlay: React.FC<SearchOverlayProps> = ({
  isOpen,
  onClose,
  onSelectResult,
}) => {
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
      document.body.style.overflow = "hidden";
    } else {
      document.body.style.overflow = "";
      setQuery("");
    }
  }, [isOpen]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        if (isOpen) onClose();
      }
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  // Demonstração editorial de busca estruturada
  const mockDemoResults: SearchResultItem[] = [
    {
      id: "collection-julie-kegels-ss27-2026",
      type: "COLLECTION",
      title: "Julie Kegels — Spring/Summer 2027",
      subtitle: "Paris Fashion Week • 28/09/2026",
      badge: "SS27",
    },
    {
      id: "city-paris",
      type: "CITY",
      title: "Paris",
      subtitle: "França • Hub Global",
      badge: "Europa",
    },
    {
      id: "event-pfw",
      type: "EVENT",
      title: "Paris Fashion Week®",
      subtitle: "FHCM • Ativo",
      badge: "Global 5",
    },
    {
      id: "maison-julie-kegels",
      type: "MAISON",
      title: "Julie Kegels",
      subtitle: "Maison Oficial • Paris",
      badge: "Maison",
    },
    {
      id: "term-trompe-loeil",
      type: "TERM",
      title: "Trompe-l'œil",
      subtitle: "Ilusão de ótica bidimensional aplicada à modelagem têxtil",
      badge: "Biblioteca",
    },
  ];

  const filteredResults = query
    ? mockDemoResults.filter(
        (r) =>
          r.title.toLowerCase().includes(query.toLowerCase()) ||
          r.subtitle?.toLowerCase().includes(query.toLowerCase())
      )
    : mockDemoResults;

  const getTypeIcon = (type: SearchResultItem["type"]) => {
    switch (type) {
      case "CITY":
        return <MapPin size={16} style={{ color: "var(--nex-accent-gold)" }} />;
      case "EVENT":
        return <Calendar size={16} style={{ color: "#60a5fa" }} />;
      case "COLLECTION":
        return <Sparkles size={16} style={{ color: "#f472b6" }} />;
      case "MAISON":
        return <Building2 size={16} style={{ color: "#a78bfa" }} />;
      case "TERM":
        return <BookMarked size={16} style={{ color: "#34d399" }} />;
    }
  };

  return (
    <div
      role="dialog"
      aria-modal="true"
      style={{
        position: "fixed",
        inset: 0,
        zIndex: "var(--nex-z-modal)",
        backgroundColor: "rgba(0, 0, 0, 0.8)",
        backdropFilter: "blur(var(--nex-blur-lg))",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        padding: "60px 16px 20px 16px",
      }}
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        style={{
          width: "100%",
          maxWidth: "640px",
          backgroundColor: "var(--nex-surface-1)",
          borderRadius: "var(--nex-radius-lg)",
          border: "1px solid var(--nex-border-strong)",
          boxShadow: "var(--nex-shadow-xl)",
          overflow: "hidden",
        }}
      >
        {/* Search Header */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            padding: "16px 20px",
            borderBottom: "1px solid var(--nex-border-subtle)",
            gap: "12px",
          }}
        >
          <Search size={20} style={{ color: "var(--nex-text-muted)" }} />
          <input
            ref={inputRef}
            type="text"
            placeholder="Buscar coleções, maisons, cidades, termos..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            style={{
              flex: 1,
              backgroundColor: "transparent",
              border: "none",
              color: "var(--nex-text-primary)",
              fontSize: "1.05rem",
              outline: "none",
            }}
          />
          {query && (
            <button
              onClick={() => setQuery("")}
              style={{
                background: "transparent",
                border: "none",
                color: "var(--nex-text-muted)",
                cursor: "pointer",
              }}
            >
              <X size={16} />
            </button>
          )}
          <span
            style={{
              fontSize: "0.72rem",
              color: "var(--nex-text-muted)",
              border: "1px solid var(--nex-border-strong)",
              borderRadius: "4px",
              padding: "2px 6px",
            }}
          >
            ESC
          </span>
        </div>

        {/* Results List */}
        <div style={{ maxHeight: "420px", overflowY: "auto", padding: "10px" }}>
          <div
            style={{
              fontSize: "0.72rem",
              color: "var(--nex-text-muted)",
              textTransform: "uppercase",
              letterSpacing: "0.06em",
              padding: "8px 12px",
              fontWeight: 600,
            }}
          >
            {query ? "Resultados da Busca" : "Sugestões Editoriais"}
          </div>

          {filteredResults.length === 0 ? (
            <div
              style={{
                padding: "32px",
                textAlign: "center",
                color: "var(--nex-text-muted)",
                fontSize: "0.9rem",
              }}
            >
              Nenhum resultado encontrado para "{query}".
            </div>
          ) : (
            filteredResults.map((item) => (
              <div
                key={item.id}
                onClick={() => {
                  onSelectResult?.(item);
                  onClose();
                }}
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                  padding: "10px 14px",
                  borderRadius: "var(--nex-radius-md)",
                  cursor: "pointer",
                  transition: "background var(--nex-transition-fast)",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.backgroundColor = "var(--nex-surface-2)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor = "transparent";
                }}
              >
                <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
                  <div
                    style={{
                      width: "32px",
                      height: "32px",
                      borderRadius: "var(--nex-radius-sm)",
                      backgroundColor: "var(--nex-surface-2)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    {getTypeIcon(item.type)}
                  </div>
                  <div>
                    <div style={{ fontSize: "0.92rem", fontWeight: 600, color: "var(--nex-text-primary)" }}>
                      {item.title}
                    </div>
                    {item.subtitle && (
                      <div style={{ fontSize: "0.76rem", color: "var(--nex-text-secondary)" }}>
                        {item.subtitle}
                      </div>
                    )}
                  </div>
                </div>

                <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                  {item.badge && (
                    <span
                      style={{
                        fontSize: "0.7rem",
                        padding: "2px 6px",
                        borderRadius: "var(--nex-radius-sm)",
                        backgroundColor: "var(--nex-surface-2)",
                        color: "var(--nex-text-muted)",
                        fontWeight: 600,
                      }}
                    >
                      {item.badge}
                    </span>
                  )}
                  <ArrowUpRight size={14} style={{ color: "var(--nex-text-muted)" }} />
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
