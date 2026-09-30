import React, { useState } from "react";
import { Search, Compass, MapPin, Globe } from "lucide-react";
import { CityCard, Chip, Badge } from "@nex-plus/ui";

export interface RotaGlobalPageProps {
  onNavigate: (route: string) => void;
}

export const RotaGlobalPage: React.FC<RotaGlobalPageProps> = ({ onNavigate }) => {
  const [selectedRegion, setSelectedRegion] = useState("Todas");
  const [searchQuery, setSearchQuery] = useState("");

  const regions = [
    "Todas",
    "Europa",
    "América do Norte",
    "América Latina",
    "Ásia",
    "Oriente Médio",
    "África",
    "Oceania",
  ];

  // Acervo canônico inicial de Cidades/Hubs semeados (136 cidades na base final)
  const canonicalCities = [
    { id: "paris", name: "Paris", country: "França", region: "Europa", eventsCount: 4, hubImportance: "Global" as const },
    { id: "milano", name: "Milão", country: "Itália", region: "Europa", eventsCount: 3, hubImportance: "Global" as const },
    { id: "london", name: "Londres", country: "Reino Unido", region: "Europa", eventsCount: 2, hubImportance: "Global" as const },
    { id: "new-york", name: "Nova York", country: "Estados Unidos", region: "América do Norte", eventsCount: 3, hubImportance: "Global" as const },
    { id: "tokyo", name: "Tóquio", country: "Japão", region: "Ásia", eventsCount: 2, hubImportance: "Global" as const },
    { id: "sao-paulo", name: "São Paulo", country: "Brasil", region: "América Latina", eventsCount: 2, hubImportance: "Nacional" as const },
    { id: "rio-de-janeiro", name: "Rio de Janeiro", country: "Brasil", region: "América Latina", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "seoul", name: "Seul", country: "Coreia do Sul", region: "Ásia", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "copenhagen", name: "Copenhague", country: "Dinamarca", region: "Europa", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "berlin", name: "Berlim", country: "Alemanha", region: "Europa", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "madrid", name: "Madri", country: "Espanha", region: "Europa", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "sydney", name: "Sydney", country: "Austrália", region: "Oceania", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "dubai", name: "Dubai", country: "Emirados Árabes", region: "Oriente Médio", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "johannesburg", name: "Joanesburgo", country: "África do Sul", region: "África", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "mexico-city", name: "Cidade do México", country: "México", region: "América Latina", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "toronto", name: "Toronto", country: "Canadá", region: "América do Norte", eventsCount: 1, hubImportance: "Regional" as const },
  ];

  const filteredCities = canonicalCities.filter((city) => {
    const matchesRegion = selectedRegion === "Todas" || city.region === selectedRegion;
    const matchesSearch =
      city.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      city.country.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesRegion && matchesSearch;
  });

  return (
    <div style={{ maxWidth: "1440px", margin: "0 auto", padding: "24px 24px 80px 24px" }}>
      {/* Page Header */}
      <div style={{ marginBottom: "32px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "8px" }}>
          <Compass size={20} style={{ color: "var(--nex-accent-gold)" }} />
          <span style={{ fontSize: "0.8rem", color: "var(--nex-accent-gold)", fontWeight: 700, textTransform: "uppercase", letterSpacing: "0.1em" }}>
            Geografia & Circuito Mundial
          </span>
        </div>

        <h1
          style={{
            fontFamily: "var(--nex-font-display)",
            fontSize: "clamp(2rem, 4vw, 3rem)",
            fontWeight: 800,
            color: "var(--nex-text-primary)",
            lineHeight: 1.1,
            marginBottom: "12px",
          }}
        >
          Rota Global da Moda
        </h1>

        <p style={{ fontSize: "1rem", color: "var(--nex-text-secondary)", maxWidth: "700px", lineHeight: 1.5 }}>
          Explore todas as cidades e polos do circuito internacional de moda. Cada hub agrupa os eventos, semanas de moda e arquivos históricos correspondentes.
        </p>
      </div>

      {/* Filter Toolbar */}
      <div
        style={{
          display: "flex",
          flexWrap: "wrap",
          justifyContent: "space-between",
          alignItems: "center",
          gap: "16px",
          marginBottom: "28px",
          borderBottom: "1px solid var(--nex-border-subtle)",
          paddingBottom: "16px",
        }}
      >
        {/* Region Filter Chips */}
        <div style={{ display: "flex", gap: "8px", flexWrap: "wrap" }}>
          {regions.map((reg) => (
            <Chip
              key={reg}
              size="md"
              active={selectedRegion === reg}
              onClick={() => setSelectedRegion(reg)}
            >
              {reg}
            </Chip>
          ))}
        </div>

        {/* Search City Input */}
        <div
          style={{
            position: "relative",
            width: "100%",
            maxWidth: "280px",
          }}
        >
          <Search
            size={16}
            style={{
              position: "absolute",
              left: "12px",
              top: "50%",
              transform: "translateY(-50%)",
              color: "var(--nex-text-muted)",
            }}
          />
          <input
            type="text"
            placeholder="Filtrar por cidade ou país..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            style={{
              width: "100%",
              padding: "8px 12px 8px 36px",
              backgroundColor: "var(--nex-surface-2)",
              border: "1px solid var(--nex-border-subtle)",
              borderRadius: "var(--nex-radius-full)",
              color: "var(--nex-text-primary)",
              fontSize: "0.85rem",
              outline: "none",
            }}
          />
        </div>
      </div>

      {/* Counter */}
      <div style={{ fontSize: "0.8rem", color: "var(--nex-text-muted)", marginBottom: "20px" }}>
        Exibindo {filteredCities.length} de {canonicalCities.length} cidades da rota global
      </div>

      {/* Grid of Cities (Proporção 2:3 portrait) */}
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(220px, 1fr))",
          gap: "22px",
        }}
      >
        {filteredCities.map((city) => (
          <CityCard
            key={city.id}
            id={city.id}
            name={city.name}
            country={city.country}
            region={city.region}
            eventsCount={city.eventsCount}
            hubImportance={city.hubImportance}
            onClick={() => {
              if (city.id === "paris") {
                onNavigate("collection");
              } else {
                alert(`Cidade ${city.name} selecionada. Mostrando eventos de ${city.name}.`);
              }
            }}
          />
        ))}
      </div>
    </div>
  );
};
