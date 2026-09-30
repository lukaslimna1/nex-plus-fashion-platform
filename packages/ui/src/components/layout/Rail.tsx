import React, { useRef, useState, useEffect } from "react";
import { ChevronLeft, ChevronRight } from "lucide-react";

export interface RailProps {
  children: React.ReactNode;
  itemGap?: number;
  className?: string;
  style?: React.CSSProperties;
}

export const Rail: React.FC<RailProps> = ({
  children,
  itemGap = 16,
  className = "",
  style,
}) => {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(true);

  const checkScroll = () => {
    if (scrollRef.current) {
      const { scrollLeft, scrollWidth, clientWidth } = scrollRef.current;
      setCanScrollLeft(scrollLeft > 10);
      setCanScrollRight(scrollLeft < scrollWidth - clientWidth - 10);
    }
  };

  useEffect(() => {
    checkScroll();
    window.addEventListener("resize", checkScroll);
    return () => window.removeEventListener("resize", checkScroll);
  }, [children]);

  const handleScroll = (direction: "left" | "right") => {
    if (scrollRef.current) {
      const scrollAmount = scrollRef.current.clientWidth * 0.75;
      scrollRef.current.scrollBy({
        left: direction === "left" ? -scrollAmount : scrollAmount,
        behavior: "smooth",
      });
      setTimeout(checkScroll, 350);
    }
  };

  return (
    <div
      style={{
        position: "relative",
        width: "100%",
        ...style,
      }}
      className={`nex-rail-wrapper ${className}`}
    >
      {/* Scroll Left Button */}
      {canScrollLeft && (
        <button
          onClick={() => handleScroll("left")}
          aria-label="Rolar para esquerda"
          style={{
            position: "absolute",
            left: "8px",
            top: "50%",
            transform: "translateY(-50%)",
            zIndex: 10,
            width: "40px",
            height: "40px",
            borderRadius: "50%",
            backgroundColor: "rgba(13, 13, 17, 0.85)",
            backdropFilter: "blur(8px)",
            border: "1px solid var(--nex-border-strong)",
            color: "var(--nex-text-primary)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            cursor: "pointer",
            boxShadow: "var(--nex-shadow-md)",
            transition: "all var(--nex-transition-fast)",
          }}
          className="nex-rail-nav-btn"
        >
          <ChevronLeft size={22} />
        </button>
      )}

      {/* Scroll Right Button */}
      {canScrollRight && (
        <button
          onClick={() => handleScroll("right")}
          aria-label="Rolar para direita"
          style={{
            position: "absolute",
            right: "8px",
            top: "50%",
            transform: "translateY(-50%)",
            zIndex: 10,
            width: "40px",
            height: "40px",
            borderRadius: "50%",
            backgroundColor: "rgba(13, 13, 17, 0.85)",
            backdropFilter: "blur(8px)",
            border: "1px solid var(--nex-border-strong)",
            color: "var(--nex-text-primary)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            cursor: "pointer",
            boxShadow: "var(--nex-shadow-md)",
            transition: "all var(--nex-transition-fast)",
          }}
          className="nex-rail-nav-btn"
        >
          <ChevronRight size={22} />
        </button>
      )}

      {/* Scrollable Container */}
      <div
        ref={scrollRef}
        onScroll={checkScroll}
        style={{
          display: "flex",
          gap: `${itemGap}px`,
          overflowX: "auto",
          scrollSnapType: "x mandatory",
          scrollBehavior: "smooth",
          padding: "8px 0 16px 0",
        }}
        className="nex-rail-container nex-no-scrollbar"
      >
        {React.Children.map(children, (child) => (
          <div
            style={{
              flexShrink: 0,
              scrollSnapAlign: "start",
            }}
          >
            {child}
          </div>
        ))}
      </div>
    </div>
  );
};

export interface SectionHeaderProps {
  title: string;
  subtitle?: string;
  badge?: React.ReactNode;
  actionText?: string;
  onActionClick?: () => void;
  className?: string;
}

export const SectionHeader: React.FC<SectionHeaderProps> = ({
  title,
  subtitle,
  badge,
  actionText,
  onActionClick,
  className = "",
}) => {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "flex-end",
        marginBottom: "16px",
      }}
      className={`nex-section-header ${className}`}
    >
      <div>
        <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
          <h2
            style={{
              fontFamily: "var(--nex-font-sans)",
              fontSize: "1.35rem",
              fontWeight: 700,
              color: "var(--nex-text-primary)",
              letterSpacing: "-0.01em",
            }}
          >
            {title}
          </h2>
          {badge}
        </div>
        {subtitle && (
          <p style={{ fontSize: "0.82rem", color: "var(--nex-text-muted)" }}>
            {subtitle}
          </p>
        )}
      </div>

      {actionText && (
        <button
          onClick={onActionClick}
          style={{
            background: "transparent",
            border: "none",
            color: "var(--nex-text-primary)",
            fontSize: "0.82rem",
            fontWeight: 600,
            cursor: "pointer",
            display: "inline-flex",
            alignItems: "center",
            gap: "4px",
            padding: "4px 8px",
            borderRadius: "var(--nex-radius-sm)",
            transition: "color var(--nex-transition-fast)",
            whiteSpace: "nowrap",
          }}
          className="nex-section-action"
        >
          <span>{actionText}</span>
          <ChevronRight size={14} />
        </button>
      )}
    </div>
  );
};
