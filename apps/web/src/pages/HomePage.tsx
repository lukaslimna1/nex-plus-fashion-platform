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
} from "@nex-plus/ui";

export interface HomePageProps {
  onNavigate: (route: string) => void;
}

export const HomePage: React.FC<HomePageProps> = ({ onNavigate }) => {
  const [heroSlide, setHeroSlide] = useState(0);
  const [selectedRegion, setSelectedRegion] = useState<string>("Todas");

  // Hero Slides: 1 fixo institucional NEX+ + 4 dinâmicos reais do catálogo
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

  // Dados reais/canônicos das Cidades/Hubs aprovadas
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

  // Maisons com seleção estável
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
      {/* Hero Section (Máx 5 slides: 1 institucional fixo + 4 dinâmicos) */}
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

      {/* Acontecendo Agora / Em Breve (Real-Time derivado do calendário oficial) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Acontecendo Agora / Em Breve"
          subtitle="Transmissões e desfiles em tempo real calculados pelo fuso oficial do evento."
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
                  Paris Fashion Week® · Womenswear Spring/Summer 2027
                </span>
                <Badge variant="live" size="sm">
                  Verificado FHCM
                </Badge>
              </div>
              <div style={{ fontSize: "0.8rem", color: "var(--nex-text-muted)" }}>
                Apresentação Julie Kegels (12:30 CEST) registrada no calendário oficial.
              </div>
            </div>
          </div>
          <Button
            variant="secondary"
            size="sm"
            onClick={() => onNavigate("collection")}
          >
            Ver Detalhes do Desfile
          </Button>
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
          {[
            {
              id: "collection-julie-kegels-ss27-2026",
              name: "Julie Kegels — Womenswear Spring/Summer 2027",
              maison: "Julie Kegels",
              seasonLabel: "Spring/Summer 2027",
              seasonCode: "SS27",
              presentedOn: "28/09/2026",
              format: "SHOW" as const,
              isPendingVerification: true,
            },
          ].map((col) => (
            <div key={col.id} style={{ width: "260px" }}>
              <CollectionCard
                id={col.id}
                name={col.name}
                maison={col.maison}
                seasonLabel={col.seasonLabel}
                seasonCode={col.seasonCode}
                presentedOn={col.presentedOn}
                format={col.format}
                isPendingVerification={col.isPendingVerification}
                onClick={() => onNavigate("collection")}
              />
            </div>
          ))}
        </Rail>
      </section>

      {/* Maisons (10 Maisons em formato compacto/circular com shuffle estável) */}
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

      {/* Críticas Recentes (Cards editoriais de publicação) */}
      <section style={{ marginBottom: "48px" }}>
        <SectionHeader
          title="Críticas Recentes"
          subtitle="Análises profissionais e resenhas críticas dos principais veículos de moda."
        />
        <Rail itemGap={18}>
          <div style={{ width: "340px" }}>
            <ReviewCard
              id="rev-1"
              publication="Vogue Runway"
              criticName="Sarah Mower"
              title="A subversão das proporções clássicas em Paris"
              excerpt="Uma abordagem meticulosa sobre a alfaiataria desconstruída, resgatando técnicas manuais belgas com um olhar cinematográfico contemporâneo para o verão de 2027."
              maison="Julie Kegels"
              collectionName="Womenswear Spring/Summer 2027"
              seasonCode="SS27"
              publishedAt="29/09/2026"
              originalUrl="https://www.vogue.com/fashion-shows"
              onOpenCollectionReview={() => onNavigate("collection")}
            />
          </div>
          <div style={{ width: "340px" }}>
            <ReviewCard
              id="rev-2"
              publication="The Business of Fashion"
              criticName="Tim Blanks"
              title="O rigor técnico encontra o repertório de arquivo"
              excerpt="A apresentação na semana de moda parisiense reafirmou o papel dos ateliês independentes na preservação da narrativa autoral frente à homogeneização do luxo."
              maison="Julie Kegels"
              collectionName="Womenswear Spring/Summer 2027"
              seasonCode="SS27"
              publishedAt="29/09/2026"
              originalUrl="https://www.businessoffashion.com"
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
