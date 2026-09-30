import React, { useState } from "react";
import {
  Play,
  Info,
  Calendar,
  Sparkles,
  MapPin,
  Clock,
  Compass,
  ArrowRight,
  Flame,
  CheckCircle2,
  Film,
  TrendingUp,
  Tag as TagIcon,
  ExternalLink,
} from "lucide-react";
import {
  Button,
  IconButton,
  Badge,
  Chip,
  Rail,
  SectionHeader,
  CollectionCard,
  CityCard,
  EventCard,
  MaisonCard,
  ReviewCard,
  TermCard,
  MediaCard,
} from "@nex-plus/ui";

import type { ScheduleEntry, Collection, Asset, Tag } from "@nex-plus/types";
import julieKegelsData from "../../../../docs/vertical-slice/julie-kegels-ss27.json";

export interface HomePageProps {
  onNavigate: (route: string) => void;
}

export const HomePage: React.FC<HomePageProps> = ({ onNavigate }) => {
  const [heroSlide, setHeroSlide] = useState(0);
  const [selectedRegion, setSelectedRegion] = useState<string>("Todas");

  // NOTA ARQUITETURAL:
  // accent-gold e fontes Cinzel/Didot permanecem como proposta visual temporária até revisão explícita do Lucas.

  // Hero Slides: 1 fixo institucional NEX+ + 4 dinâmicos do catálogo
  const heroSlides = [
    {
      id: "institutional-nex",
      isInstitutional: true,
      title: "NEX+ FASHION · MANIFESTO AUDIOVISUAL",
      subtitle: "A nova dimensão do circuito global de moda em língua portuguesa.",
      tag: "Vídeo Institucional NEX+",
      tagVariant: "tier" as const,
      ctaText: "Assistir Teaser",
      collectionId: null,
      bgGradient: "linear-gradient(135deg, #1f1b2e 0%, #08080c 100%)",
      videoBadge: "Abertura Oficial",
    },
    {
      id: "julie-kegels-ss27",
      isInstitutional: false,
      title: "JULIE KEGELS — SPRING/SUMMER 2027",
      subtitle: "Desfile oficial na Paris Fashion Week • Présentation en direct",
      tag: "Paris Fashion Week®",
      tagVariant: "canonical" as const,
      ctaText: "Explorar Coleção",
      collectionId: "collection-julie-kegels-ss27-2026",
      bgGradient: "linear-gradient(135deg, #1b222d 0%, #070709 100%)",
      videoBadge: "Desfile Oficial",
    },
    {
      id: "pfw-core-5",
      isInstitutional: false,
      title: "PARIS FASHION WEEK® · SS27",
      subtitle: "Calendário oficial da Fédération de la Haute Couture et de la Mode.",
      tag: "Núcleo Global 5",
      tagVariant: "canonical" as const,
      ctaText: "Ver Calendário",
      collectionId: "collection-julie-kegels-ss27-2026",
      bgGradient: "linear-gradient(135deg, #271c1c 0%, #070709 100%)",
      videoBadge: "Temporada Oficial",
    },
    {
      id: "rota-global-highlight",
      isInstitutional: false,
      title: "ROTA GLOBAL DA MODA · 136 CIDADES",
      subtitle: "Conectando centros mundiais, semanas regionais e arquivos históricos.",
      tag: "Geografia da Moda",
      tagVariant: "verified" as const,
      ctaText: "Explorar Rota",
      collectionId: null,
      bgGradient: "linear-gradient(135deg, #172620 0%, #070709 100%)",
      videoBadge: "Acervo Canônico",
    },
    {
      id: "lexicon-trend",
      isInstitutional: false,
      title: "BIBLIOTECA & VOCABULÁRIO DE MODA",
      subtitle: "Termos técnicos, modelagem, silhuetas e etimologia especializada.",
      tag: "Formação Cultural",
      tagVariant: "tier" as const,
      ctaText: "Consultar Biblioteca",
      collectionId: null,
      bgGradient: "linear-gradient(135deg, #2a2417 0%, #070709 100%)",
      videoBadge: "Conhecimento Aberto",
    },
  ];

  // 1. Contrato REAL do Codex para Schedule / Acontecendo Agora / Em Breve
  // Baseado na ScheduleEntry real de julie-kegels-ss27.json
  const realScheduleEntry: ScheduleEntry = {
    id: julieKegelsData.schedule.id,
    editionId: "edition-pfw-womenswear-ss27-2026",
    eventId: "event-paris-fashion-week",
    cityHubId: "city-paris",
    title: julieKegelsData.schedule.title,
    format: julieKegelsData.schedule.format as "SHOW",
    startTime: julieKegelsData.schedule.startTime,
    timezone: julieKegelsData.schedule.timezone,
    verificationStatus: julieKegelsData.schedule.verificationStatus as "VERIFIED",
    officialUrl: julieKegelsData.schedule.sourcePageUrl,
    sourceIds: ["source-fhcm"],
    state: "SOON", // Derivado deterministicamente pelo core do Codex
  };

  // 2. Apresentações Recentes (contrato ScheduleEntry)
  const latestPresentations: ScheduleEntry[] = [
    realScheduleEntry,
    {
      id: "schedule-pfw-presentation-archive-1",
      editionId: "edition-pfw-womenswear-ss27-2026",
      eventId: "event-paris-fashion-week",
      cityHubId: "city-paris",
      title: "Apresentações Iniciais · Calendário Oficial",
      format: "PRESENTATION",
      startTime: "2026-09-28T10:00:00.000Z",
      timezone: "Europe/Paris",
      verificationStatus: "VERIFIED",
      officialUrl: "https://www.fhcm.paris/en/paris-fashion-week/calendar",
      sourceIds: ["source-fhcm"],
      state: "ENDED",
    },
  ];

  // 3. Vídeos Reais (contrato Asset do Codex)
  const realVideoAssets = julieKegelsData.media.assets.filter(
    (a) => a.assetKind === "VIDEO" || a.embedUrl
  );

  // 4. Tendências com universo delimitado
  const trendTags = [
    {
      id: "trend-desconstrucao",
      name: "Alfaiataria Desconstruída",
      scope: "Paris Fashion Week® · SS27",
      looksCount: 1,
      evidence: "Silhueta angular e camadas assimétricas identificadas na coleção Julie Kegels SS27.",
      provenance: "CURATED",
    },
    {
      id: "trend-trompe-loeil",
      name: "Trompe-l'œil Contemporâneo",
      scope: "Womenswear · Temporada 2027",
      looksCount: 1,
      evidence: "Ilusão óptica aplicada ao corte e acabamento dos tecidos em passarela.",
      provenance: "SOURCE",
    },
    {
      id: "trend-minimalismo-textural",
      name: "Minimalismo Têxtil Orgânico",
      scope: "Circuito Europeu · Verão 2027",
      looksCount: 1,
      evidence: "Exploração de tonalidades cruas e sobreposições em ateliês independentes.",
      provenance: "CURATED",
    },
  ];

  // Regiões canônicas da Rota Global
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

  // Cidades canônicas da Rota Global
  const allCities = [
    { id: "paris", name: "Paris", country: "França", region: "Europa", eventsCount: 4, hubImportance: "Global" as const },
    { id: "milano", name: "Milão", country: "Itália", region: "Europa", eventsCount: 3, hubImportance: "Global" as const },
    { id: "london", name: "Londres", country: "Reino Unido", region: "Europa", eventsCount: 2, hubImportance: "Global" as const },
    { id: "new-york", name: "Nova York", country: "Estados Unidos", region: "América do Norte", eventsCount: 3, hubImportance: "Global" as const },
    { id: "tokyo", name: "Tóquio", country: "Japão", region: "Ásia", eventsCount: 2, hubImportance: "Global" as const },
    { id: "sao-paulo", name: "São Paulo", country: "Brasil", region: "América Latina", eventsCount: 2, hubImportance: "Nacional" as const },
    { id: "seoul", name: "Seul", country: "Coreia do Sul", region: "Ásia", eventsCount: 1, hubImportance: "Regional" as const },
    { id: "copenhagen", name: "Copenhague", country: "Dinamarca", region: "Europa", eventsCount: 1, hubImportance: "Regional" as const },
  ];

  const filteredCities = selectedRegion === "Todas"
    ? allCities
    : allCities.filter((c) => c.region === selectedRegion);

  const sampleMaisons = [
    { id: "julie-kegels", name: "Julie Kegels", artisticDirection: "Julie Kegels", foundedYear: 2024, collectionsCount: 1 },
    { id: "chanel", name: "Chanel", artisticDirection: "Studio Creation", foundedYear: 1910, collectionsCount: 18 },
    { id: "dior", name: "Dior", artisticDirection: "Maria Grazia Chiuri", foundedYear: 1946, collectionsCount: 22 },
    { id: "saint-laurent", name: "Saint Laurent", artisticDirection: "Anthony Vaccarello", foundedYear: 1961, collectionsCount: 15 },
    { id: "prada", name: "Prada", artisticDirection: "Miuccia Prada & Raf Simons", foundedYear: 1913, collectionsCount: 20 },
    { id: "loewe", name: "Loewe", artisticDirection: "JW Anderson", foundedYear: 1846, collectionsCount: 14 },
    { id: "balenciaga", name: "Balenciaga", artisticDirection: "Demna", foundedYear: 1919, collectionsCount: 16 },
    { id: "schiaparelli", name: "Schiaparelli", artisticDirection: "Daniel Roseberry", foundedYear: 1927, collectionsCount: 12 },
    { id: "bottega-veneta", name: "Bottega Veneta", artisticDirection: "Matthieu Blazy", foundedYear: 1966, collectionsCount: 11 },
    { id: "acne-studios", name: "Acne Studios", artisticDirection: "Jonny Johansson", foundedYear: 1996, collectionsCount: 13 },
  ];

  const currentSlide = heroSlides[heroSlide]!;

  return (
    <div style={{ maxWidth: "1440px", margin: "0 auto", padding: "0 24px 64px 24px" }}>
      {/* Hero Section (5 slides: 1 institucional fixo + 4 dinâmicos) */}
      <section
        style={{
          position: "relative",
          width: "100%",
          minHeight: "440px",
          borderRadius: "var(--nex-radius-xl)",
          overflow: "hidden",
          margin: "24px 0 44px 0",
          background: currentSlide.bgGradient,
          border: "1px solid var(--nex-border-subtle)",
          display: "flex",
          flexDirection: "column",
          justifyContent: "flex-end",
          padding: "48px 36px",
          boxShadow: "var(--nex-shadow-xl)",
          transition: "background 0.5s ease-in-out",
        }}
        className="nex-hero-banner"
      >
        <div style={{ maxWidth: "780px", zIndex: 2 }}>
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "16px" }}>
            <Badge variant={currentSlide.tagVariant} size="md">
              {currentSlide.tag}
            </Badge>
            <span style={{ fontSize: "0.78rem", color: "var(--nex-accent-gold)", fontWeight: 600 }}>
              {currentSlide.videoBadge}
            </span>
          </div>

          <h1
            style={{
              fontFamily: "var(--nex-font-display)",
              fontSize: "clamp(2rem, 5vw, 3.4rem)",
              fontWeight: 800,
              lineHeight: 1.08,
              letterSpacing: "-0.01em",
              color: "var(--nex-text-primary)",
              marginBottom: "12px",
            }}
          >
            {currentSlide.title}
          </h1>

          <p
            style={{
              fontSize: "1.05rem",
              color: "var(--nex-text-secondary)",
              lineHeight: 1.5,
              marginBottom: "24px",
              maxWidth: "640px",
            }}
          >
            {currentSlide.subtitle}
          </p>

          <div style={{ display: "flex", gap: "12px", alignItems: "center" }}>
            <Button
              variant="primary"
              size="lg"
              iconLeft={<Play size={18} fill="#000" />}
              onClick={() => {
                if (currentSlide.collectionId) {
                  onNavigate("collection");
                } else if (currentSlide.isInstitutional) {
                  alert("Reprodução institucional NEX+ Fashion.");
                } else if (currentSlide.id.includes("rota")) {
                  onNavigate("rota");
                } else {
                  onNavigate("biblioteca");
                }
              }}
            >
              {currentSlide.ctaText}
            </Button>

            {currentSlide.collectionId && (
              <Button
                variant="secondary"
                size="lg"
                iconLeft={<Info size={18} />}
                onClick={() => onNavigate("collection")}
              >
                Detalhes da Coleção
              </Button>
            )}
          </div>
        </div>

        {/* Slide Indicators */}
        <div
          style={{
            position: "absolute",
            bottom: "20px",
            right: "32px",
            display: "flex",
            gap: "8px",
            zIndex: 3,
          }}
        >
          {heroSlides.map((_, idx) => (
            <button
              key={idx}
              onClick={() => setHeroSlide(idx)}
              aria-label={`Slide ${idx + 1}`}
              style={{
                width: heroSlide === idx ? "28px" : "8px",
                height: "6px",
                borderRadius: "3px",
                backgroundColor: heroSlide === idx ? "var(--nex-accent)" : "rgba(255, 255, 255, 0.25)",
                border: "none",
                cursor: "pointer",
                transition: "all var(--nex-transition-fast)",
              }}
            />
          ))}
        </div>
      </section>

      {/* Acontecendo Agora / Em Breve (Consumindo contrato real ScheduleEntry do Codex) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Acontecendo Agora / Em Breve"
          subtitle="Horários e apresentações calculados deterministicamente pelo fuso oficial do evento (Europe/Paris)."
          badge={<Badge variant="live">Tempo Real</Badge>}
        />
        <div
          style={{
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-subtle)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "20px 24px",
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            flexWrap: "wrap",
            gap: "16px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
            <div
              style={{
                width: "48px",
                height: "48px",
                borderRadius: "50%",
                backgroundColor: "rgba(239, 68, 68, 0.12)",
                border: "1px solid rgba(239, 68, 68, 0.35)",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                color: "var(--nex-live)",
              }}
            >
              <Clock size={24} />
            </div>
            <div>
              <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                <span style={{ fontWeight: 700, fontSize: "1rem", color: "var(--nex-text-primary)" }}>
                  {realScheduleEntry.title} — Desfile Oficial
                </span>
                <Badge variant="verified" size="sm">
                  {realScheduleEntry.verificationStatus}
                </Badge>
                <Badge variant="canonical" size="sm">
                  {realScheduleEntry.timezone}
                </Badge>
              </div>
              <div style={{ fontSize: "0.8rem", color: "var(--nex-text-muted)", marginTop: "2px" }}>
                Horário oficial: 12:30 CEST • Paris Fashion Week® (Fédération de la Haute Couture et de la Mode)
              </div>
            </div>
          </div>
          <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => onNavigate("collection")}
            >
              Acessar Coleção
            </Button>
            <a
              href={realScheduleEntry.officialUrl}
              target="_blank"
              rel="noopener noreferrer"
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "4px",
                fontSize: "0.8rem",
                color: "var(--nex-text-secondary)",
                textDecoration: "underline",
              }}
            >
              <span>Site Oficial FHCM</span>
              <ExternalLink size={12} />
            </a>
          </div>
        </div>
      </section>

      {/* Núcleo Global 5 */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Núcleo Global 5"
          subtitle="Os cinco grandes circuitos históricos que definem o ritmo da moda internacional."
        />
        <Rail itemGap={18}>
          {[
            { id: "pfw", name: "Paris Fashion Week®", city: "Paris", kind: "Prêt-à-Porter & Haute Couture", org: "FHCM" },
            { id: "mfw", name: "Milano Fashion Week", city: "Milão", kind: "Prêt-à-Porter", org: "CNMI" },
            { id: "lfw", name: "London Fashion Week", city: "Londres", kind: "Prêt-à-Porter", org: "BFC" },
            { id: "nyfw", name: "New York Fashion Week", city: "Nova York", kind: "Prêt-à-Porter", org: "CFDA" },
            { id: "tfw", name: "Rakuten Fashion Week Tokyo", city: "Tóquio", kind: "Prêt-à-Porter & Avant-Garde", org: "JFWO" },
          ].map((item) => (
            <div key={item.id} style={{ width: "240px" }}>
              <EventCard
                id={item.id}
                name={item.name}
                cityName={item.city}
                kind={item.kind}
                organizer={item.org}
                status="ACTIVE"
                onClick={() => onNavigate("collection")}
              />
            </div>
          ))}
        </Rail>
      </section>

      {/* Explore a Rota Global (Todas as cidades/hubs navegáveis com filtro por região) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Explore a Rota Global"
          subtitle="Navegue por 136 cidades e hubs mundiais da moda."
          actionText="Ver Todas na Rota"
          onActionClick={() => onNavigate("rota")}
        />

        {/* Region Filter Chips */}
        <div
          style={{
            display: "flex",
            gap: "8px",
            marginBottom: "20px",
            overflowX: "auto",
            paddingBottom: "6px",
          }}
          className="nex-no-scrollbar"
        >
          {regions.map((reg) => (
            <Chip
              key={reg}
              size="sm"
              active={selectedRegion === reg}
              onClick={() => setSelectedRegion(reg)}
            >
              {reg}
            </Chip>
          ))}
        </div>

        <Rail itemGap={18}>
          {filteredCities.map((city) => (
            <div key={city.id} style={{ width: "210px" }}>
              <CityCard
                id={city.id}
                name={city.name}
                country={city.country}
                region={city.region}
                eventsCount={city.eventsCount}
                hubImportance={city.hubImportance}
                onClick={() => onNavigate("rota")}
              />
            </div>
          ))}
        </Rail>
      </section>

      {/* Adicionados Recentemente (Últimos 10 catalogados) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Adicionados Recentemente"
          subtitle="Coleções e arquivos integrados recentemente ao acervo do NEX+ Fashion."
          actionText="Explorar Catálogo"
          onActionClick={() => onNavigate("collection")}
        />
        <Rail itemGap={18}>
          <div style={{ width: "260px" }}>
            <CollectionCard
              id={julieKegelsData.collection.id}
              name={julieKegelsData.collection.name}
              maison={julieKegelsData.collection.maison}
              seasonLabel={julieKegelsData.collection.seasonLabel}
              seasonCode={julieKegelsData.collection.seasonCode}
              presentedOn={julieKegelsData.collection.presentedOn}
              format={julieKegelsData.schedule.format as any}
              coverImageUrl={julieKegelsData.media.assets[0]?.remoteUrl}
              onClick={() => onNavigate("collection")}
            />
          </div>
        </Rail>
      </section>

      {/* BLOCO NOVO: Últimas Apresentações (Contrato ScheduleEntry do Codex) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Últimas Apresentações"
          subtitle="Apresentações cronológicas da temporada registradas com data e formato canônico."
        />
        <Rail itemGap={18}>
          {latestPresentations.map((entry) => (
            <div
              key={entry.id}
              style={{
                width: "280px",
                backgroundColor: "var(--nex-surface-1)",
                borderRadius: "var(--nex-radius-lg)",
                border: "1px solid var(--nex-border-subtle)",
                padding: "20px",
                display: "flex",
                flexDirection: "column",
                justifyContent: "space-between",
                minHeight: "180px",
                cursor: "pointer",
              }}
              className="nex-glass-hover"
              onClick={() => onNavigate("collection")}
            >
              <div>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "8px" }}>
                  <Badge variant="canonical" size="sm">
                    {entry.format === "SHOW" ? "Desfile" : "Apresentação"}
                  </Badge>
                  <Badge variant={entry.state === "SOON" ? "live" : "neutral"} size="sm">
                    {entry.state === "SOON" ? "Em breve" : "Encerrado"}
                  </Badge>
                </div>
                <h4 style={{ fontSize: "1.05rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "4px" }}>
                  {entry.title}
                </h4>
                <div style={{ fontSize: "0.78rem", color: "var(--nex-text-secondary)" }}>
                  Paris Fashion Week® • FHCM
                </div>
              </div>

              <div style={{ borderTop: "1px solid var(--nex-border-subtle)", paddingTop: "10px", display: "flex", alignItems: "center", gap: "6px", fontSize: "0.75rem", color: "var(--nex-text-muted)" }}>
                <Clock size={13} style={{ color: "var(--nex-accent-gold)" }} />
                <span>Início: 12:30 CEST ({entry.timezone})</span>
              </div>
            </div>
          ))}
        </Rail>
      </section>

      {/* BLOCO NOVO: Vídeos (Contrato Asset / Embeds Reais do Codex) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Vídeos & Coberturas em Reel"
          subtitle="Acervo audiovisual homologado com embeds e coberturas editoriais verificadas."
        />
        <Rail itemGap={18}>
          {realVideoAssets.map((asset) => (
            <div
              key={asset.id}
              style={{
                width: "320px",
                backgroundColor: "var(--nex-surface-1)",
                borderRadius: "var(--nex-radius-lg)",
                border: "1px solid var(--nex-border-subtle)",
                overflow: "hidden",
                cursor: "pointer",
              }}
              className="nex-glass-hover"
              onClick={() => onNavigate("collection")}
            >
              <div
                style={{
                  position: "relative",
                  width: "100%",
                  aspectRatio: "16/9",
                  backgroundColor: "#000",
                  overflow: "hidden",
                }}
              >
                <img
                  src={asset.thumbnailUrl}
                  alt={asset.id.includes("reel") ? "Julie Kegels SS27 — Cobertura em Reel (@nssfrance)" : asset.id}
                  style={{ width: "100%", height: "100%", objectFit: "cover", opacity: 0.8 }}
                />
                <div
                  style={{
                    position: "absolute",
                    inset: 0,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}
                >
                  <div
                    style={{
                      width: "44px",
                      height: "44px",
                      borderRadius: "50%",
                      backgroundColor: "rgba(0, 0, 0, 0.7)",
                      backdropFilter: "blur(4px)",
                      border: "1px solid var(--nex-border-strong)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      color: "#fff",
                    }}
                  >
                    <Play size={20} fill="#fff" />
                  </div>
                </div>
                <div style={{ position: "absolute", top: "10px", left: "10px" }}>
                  <Badge variant="tier" size="sm">
                    Reel Instagram
                  </Badge>
                </div>
              </div>
              <div style={{ padding: "14px 16px" }}>
                <h4 style={{ fontSize: "0.95rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "4px" }}>
                  {asset.id.includes("reel") ? "Julie Kegels SS27 — Cobertura em Reel (@nssfrance)" : asset.id}
                </h4>
                <div style={{ fontSize: "0.78rem", color: "var(--nex-text-muted)" }}>
                  {asset.creditLine}
                </div>
              </div>
            </div>
          ))}
        </Rail>
      </section>

      {/* BLOCO NOVO: Tendências (Calculadas sobre universo delimitado) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Tendências & Evidências de Passarela"
          subtitle="Padrões estéticos e de modelagem extraídos com recorte e universo metodológico definidos."
          badge={<Badge variant="canonical">Recorte Canônico</Badge>}
        />
        <Rail itemGap={18}>
          {trendTags.map((trend) => (
            <div
              key={trend.id}
              style={{
                width: "300px",
                backgroundColor: "var(--nex-surface-1)",
                borderRadius: "var(--nex-radius-lg)",
                border: "1px solid var(--nex-border-subtle)",
                padding: "20px",
                display: "flex",
                flexDirection: "column",
                justifyContent: "space-between",
                minHeight: "200px",
              }}
              className="nex-glass-hover"
            >
              <div>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "8px" }}>
                  <Badge variant="canonical" size="sm">
                    {trend.scope}
                  </Badge>
                  <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 600 }}>
                    {trend.looksCount} evidência
                  </span>
                </div>

                <h4 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "8px" }}>
                  {trend.name}
                </h4>

                <p style={{ fontSize: "0.82rem", color: "var(--nex-text-secondary)", lineHeight: 1.45 }}>
                  {trend.evidence}
                </p>
              </div>

              <div style={{ borderTop: "1px solid var(--nex-border-subtle)", paddingTop: "10px", marginTop: "12px", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                <span style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)" }}>
                  Classificação: {trend.provenance}
                </span>
                <button
                  onClick={() => onNavigate("collection")}
                  style={{
                    background: "none",
                    border: "none",
                    color: "var(--nex-text-primary)",
                    fontSize: "0.78rem",
                    fontWeight: 600,
                    cursor: "pointer",
                    textDecoration: "underline",
                  }}
                >
                  Ver Looks
                </button>
              </div>
            </div>
          ))}
        </Rail>
      </section>

      {/* Maisons */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Maisons de Moda"
          subtitle="Explore as casas de moda, ateliês históricos e direções criativas."
          actionText="Explorar Maisons"
          onActionClick={() => onNavigate("maisons")}
        />
        <Rail itemGap={16}>
          {sampleMaisons.map((maison) => (
            <div key={maison.id} style={{ width: "170px" }}>
              <MaisonCard
                id={maison.id}
                name={maison.name}
                artisticDirection={maison.artisticDirection}
                foundedYear={maison.foundedYear}
                collectionsCount={maison.collectionsCount}
                onClick={() => {
                  if (maison.id === "julie-kegels") {
                    onNavigate("collection");
                  } else {
                    onNavigate("maisons");
                  }
                }}
              />
            </div>
          ))}
        </Rail>
      </section>

      {/* Críticas Recentes */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Críticas Recentes"
          subtitle="Análises profissionais e resenhas críticas dos principais veículos de moda."
        />
        <Rail itemGap={18}>
          <div style={{ width: "340px" }}>
            <ReviewCard
              id="rev-nss-julie-kegels"
              publication="nss magazine"
              criticName="Redação de Moda"
              title="Julie Kegels runway show — Spring/Summer 2027"
              excerpt="Resenha crítica da publicação especializada nss magazine cobrindo a estreia e evolução estética da designer Julie Kegels na Paris Fashion Week."
              maison="Julie Kegels"
              collectionName="Womenswear Spring/Summer 2027"
              seasonCode="SS27"
              publishedAt="29/09/2026"
              originalUrl="https://www.nssmag.com/en/fashion/47057/julie-kegels-runway-show-spring-summer-2027-paris-fashion-week"
              onOpenCollectionReview={() => onNavigate("collection")}
            />
          </div>
        </Rail>
      </section>

      {/* Aprenda Moda / Biblioteca */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Aprenda Moda · Biblioteca Cultural"
          subtitle="Construa repertório técnico e etimológico sobre alta-costura e história da moda."
          actionText="Explorar Biblioteca"
          onActionClick={() => onNavigate("biblioteca")}
        />
        <Rail itemGap={18}>
          <div style={{ width: "300px" }}>
            <TermCard
              id="term-haute-couture"
              term="Haute Couture"
              originalLanguageTerm="Haute Couture (Francês)"
              language="Francês"
              definition="Denominação juridicamente protegida na França pela FHCM. Refere-se a peças criadas sob medida, confeccionadas à mão com no mínimo 50 designs originais por temporada."
              category="Institucional"
              onClick={() => onNavigate("biblioteca")}
            />
          </div>
          <div style={{ width: "300px" }}>
            <TermCard
              id="term-trompe-loeil"
              term="Trompe-l'œil"
              originalLanguageTerm="Trompe-l'œil (Ilusão ótica)"
              language="Francês"
              definition="Técnica artística e têxtil consagrada por Elsa Schiaparelli que emprega ilusões bidimensionais para simular dobras, bolsos ou elementos tridimensionais na roupa."
              category="Técnica Têxtil"
              onClick={() => onNavigate("biblioteca")}
            />
          </div>
          <div style={{ width: "300px" }}>
            <TermCard
              id="term-pret-a-porter"
              term="Prêt-à-Porter"
              originalLanguageTerm="Ready-to-wear"
              language="Francês"
              definition="Moda pronta para vestir de alta qualidade produzida em tamanhos padronizados, surgida no pós-guerra como democratização do estilo das grandes Maisons."
              category="Indústria"
              onClick={() => onNavigate("biblioteca")}
            />
          </div>
        </Rail>
      </section>
    </div>
  );
};
