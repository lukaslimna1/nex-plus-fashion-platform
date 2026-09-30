import React, { useState, useEffect } from "react";
import {
  Calendar,
  Clock,
  MapPin,
  ExternalLink,
  ShieldCheck,
  ShieldAlert,
  BookOpen,
  Film,
  Camera,
  Layers,
  Sparkles,
  Edit3,
  Bookmark,
  Share2,
} from "lucide-react";
import {
  Badge,
  Button,
  IconButton,
  Chip,
  EmptyState,
  SourceLink,
  MetadataLine,
} from "@nex-plus/ui";

// Importando o contrato real da vertical slice produzido pelo Codex
import julieKegelsData from "../../../../docs/vertical-slice/julie-kegels-ss27.json";

export const CollectionPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<
    "looks" | "leitura" | "critica" | "videos" | "vocabulario" | "fontes"
  >("looks");

  // Minha Leitura (local-first, sem textos preenchidos automaticamente conforme regra canônica!)
  const [cadernoNote, setCadernoNote] = useState<string>(() => {
    return localStorage.getItem("caderno_julie_kegels_ss27") || "";
  });
  const [isSaved, setIsSaved] = useState(false);

  const handleSaveNote = () => {
    localStorage.setItem("caderno_julie_kegels_ss27", cadernoNote);
    setIsSaved(true);
    setTimeout(() => setIsSaved(false), 2000);
  };

  const { collection, schedule, sources, media } = julieKegelsData;
  const isPendingVerification = media.status === "PENDING_SOURCE_VERIFICATION";

  return (
    <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "24px 20px 80px 20px" }}>
      {/* Breadcrumb Editorial */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "8px",
          fontSize: "0.78rem",
          color: "var(--nex-text-muted)",
          marginBottom: "20px",
          textTransform: "uppercase",
          letterSpacing: "0.06em",
        }}
      >
        <span>Paris</span>
        <span>/</span>
        <span>Paris Fashion Week®</span>
        <span>/</span>
        <span style={{ color: "var(--nex-accent-gold)", fontWeight: 600 }}>
          {collection.seasonCode}
        </span>
        <span>/</span>
        <span style={{ color: "var(--nex-text-primary)" }}>{collection.maison}</span>
      </div>

      {/* Collection Hero Banner */}
      <div
        style={{
          position: "relative",
          borderRadius: "var(--nex-radius-xl)",
          overflow: "hidden",
          backgroundColor: "var(--nex-surface-1)",
          border: "1px solid var(--nex-border-subtle)",
          padding: "48px 36px",
          marginBottom: "36px",
          background: "linear-gradient(135deg, #181822 0%, #0c0c10 100%)",
        }}
        className="nex-collection-hero"
      >
        <div style={{ maxWidth: "800px" }}>
          {/* Status & Badges */}
          <div style={{ display: "flex", flexWrap: "wrap", gap: "8px", marginBottom: "16px" }}>
            <Badge variant="canonical">{collection.seasonLabel}</Badge>
            <Badge variant="verified">Temporada Editorial {collection.seasonYear}</Badge>
            <Badge variant="neutral">Realizado em {collection.calendarYear}</Badge>
            <Badge variant="neutral">Formato: {schedule.format === "SHOW" ? "Desfile Oficial" : schedule.format}</Badge>
          </div>

          <div
            style={{
              fontSize: "0.95rem",
              fontFamily: "var(--nex-font-sans)",
              color: "var(--nex-accent-gold)",
              fontWeight: 700,
              textTransform: "uppercase",
              letterSpacing: "0.1em",
              marginBottom: "8px",
            }}
          >
            {collection.maison}
          </div>

          <h1
            style={{
              fontFamily: "var(--nex-font-display)",
              fontSize: "clamp(2rem, 5vw, 3.2rem)",
              fontWeight: 800,
              color: "var(--nex-text-primary)",
              lineHeight: 1.1,
              marginBottom: "16px",
              letterSpacing: "-0.01em",
            }}
          >
            {collection.name}
          </h1>

          {/* Schedule & Event Information */}
          <div
            style={{
              display: "flex",
              flexWrap: "wrap",
              gap: "20px",
              fontSize: "0.85rem",
              color: "var(--nex-text-secondary)",
              marginBottom: "24px",
            }}
          >
            <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
              <Calendar size={15} style={{ color: "var(--nex-accent-gold)" }} />
              <span>Apresentado em {collection.presentedOn}</span>
            </div>
            <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
              <Clock size={15} style={{ color: "var(--nex-text-muted)" }} />
              <span>Horário oficial: 12:30 CEST (Paris)</span>
            </div>
            <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
              <MapPin size={15} style={{ color: "var(--nex-text-muted)" }} />
              <span>Paris Fashion Week® • FHCM</span>
            </div>
          </div>

          {/* Action CTAs */}
          <div style={{ display: "flex", gap: "12px", alignItems: "center" }}>
            <Button
              variant="primary"
              size="md"
              iconLeft={<Bookmark size={16} />}
              onClick={() => alert("Coleção salva nos seus Favoritos!")}
            >
              Favoritar Coleção
            </Button>
            <Button
              variant="secondary"
              size="md"
              iconLeft={<Edit3 size={16} />}
              onClick={() => setActiveTab("leitura")}
            >
              Minha Leitura
            </Button>
          </div>
        </div>
      </div>

      {/* Elegant Warning Banner for PENDING_SOURCE_VERIFICATION */}
      {isPendingVerification && (
        <div
          style={{
            backgroundColor: "rgba(212, 175, 55, 0.08)",
            border: "1px solid rgba(212, 175, 55, 0.28)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "20px 24px",
            marginBottom: "32px",
            display: "flex",
            alignItems: "flex-start",
            gap: "16px",
          }}
        >
          <ShieldAlert size={26} style={{ color: "var(--nex-accent-gold)", flexShrink: 0, marginTop: "2px" }} />
          <div>
            <div
              style={{
                fontSize: "0.95rem",
                fontWeight: 600,
                color: "var(--nex-text-primary)",
                marginBottom: "4px",
              }}
            >
              Mídia em Verificação Canônica de Fonte (PENDING_SOURCE_VERIFICATION)
            </div>
            <p
              style={{
                fontSize: "0.85rem",
                color: "var(--nex-text-secondary)",
                lineHeight: 1.5,
                marginBottom: "8px",
              }}
            >
              {media.note}
            </p>
            <div
              style={{
                fontSize: "0.78rem",
                color: "var(--nex-accent-gold)",
                fontWeight: 500,
              }}
            >
              Próximo passo: {media.nextStep}
            </div>
          </div>
        </div>
      )}

      {/* Tabs Navigation */}
      <div
        style={{
          display: "flex",
          gap: "8px",
          borderBottom: "1px solid var(--nex-border-subtle)",
          paddingBottom: "12px",
          marginBottom: "28px",
          overflowX: "auto",
        }}
        className="nex-no-scrollbar"
      >
        {[
          { id: "looks", label: "Looks do Desfile (0)" },
          { id: "leitura", label: "Minha Leitura / Caderno" },
          { id: "critica", label: "Crítica Especializada" },
          { id: "videos", label: "Vídeos & Apresentação" },
          { id: "vocabulario", label: "Vocabulário Relacionado" },
          { id: "fontes", label: "Fontes & Proveniência" },
        ].map((tab) => (
          <Chip
            key={tab.id}
            active={activeTab === tab.id}
            onClick={() => setActiveTab(tab.id as any)}
          >
            {tab.label}
          </Chip>
        ))}
      </div>

      {/* Tab Content */}
      {activeTab === "looks" && (
        <div>
          <EmptyState
            title="Nenhum look fotográfico adicionado sem verificação prévia"
            description="O NEX+ segue a política rigorosa de não clonar mídias de terceiros nem apresentar conteúdo não homologado. Assim que a cobertura da FHCM ou fontes credenciadas forem verificadas pelo pipeline de ingestão, as fotos dos looks aparecerão aqui."
            actionText="Consultar Calendário Oficial FHCM"
            onActionClick={() => window.open(schedule.sourcePageUrl, "_blank")}
          />
        </div>
      )}

      {activeTab === "leitura" && (
        <div
          style={{
            backgroundColor: "var(--nex-surface-1)",
            border: "1px solid var(--nex-border-subtle)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "24px",
            maxWidth: "800px",
          }}
        >
          <div style={{ marginBottom: "16px" }}>
            <h3 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "4px" }}>
              Minha Leitura • Caderno Pessoal
            </h3>
            <p style={{ fontSize: "0.82rem", color: "var(--nex-text-muted)" }}>
              Seu espaço privado e local para anotações críticas, análise de silhuetas, materiais e conexões históricas desta coleção. Não é compartilhado publicamente.
            </p>
          </div>

          <textarea
            placeholder="Digite suas impressões sobre a coleção Julie Kegels SS27 (modelagem, referências, cartela de cores, texturas)..."
            value={cadernoNote}
            onChange={(e) => setCadernoNote(e.target.value)}
            style={{
              width: "100%",
              minHeight: "220px",
              backgroundColor: "var(--nex-surface-2)",
              border: "1px solid var(--nex-border-strong)",
              borderRadius: "var(--nex-radius-md)",
              padding: "16px",
              color: "var(--nex-text-primary)",
              fontFamily: "var(--nex-font-sans)",
              fontSize: "0.92rem",
              lineHeight: 1.6,
              resize: "vertical",
              outline: "none",
              marginBottom: "16px",
            }}
          />

          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
            <span style={{ fontSize: "0.78rem", color: isSaved ? "var(--nex-success)" : "var(--nex-text-muted)" }}>
              {isSaved ? "✓ Anotação salva localmente no navegador." : "Armazenamento local-first persistente."}
            </span>
            <Button variant="primary" size="sm" onClick={handleSaveNote}>
              Salvar Anotação
            </Button>
          </div>
        </div>
      )}

      {activeTab === "critica" && (
        <div style={{ maxWidth: "800px" }}>
          <EmptyState
            title="Aguardando publicação de críticas verificadas"
            description="As resenhas e análises de veículos profissionais da moda (como Vogue Runway, BoF, WWD) sobre a apresentação de Julie Kegels na Paris Fashion Week SS27 serão catalogadas preservando o autor, resumo e link para a publicação original."
          />
        </div>
      )}

      {activeTab === "videos" && (
        <div style={{ maxWidth: "800px" }}>
          <EmptyState
            title="Transmissão oficial em verificação"
            description="Nenhum embed de vídeo ou livestream não oficial foi detectado com licença de incorporação para esta edição. Ao ser confirmado pela Maison ou FHCM, o player interno estará disponível."
            actionText="Verificar no Portal FHCM"
            onActionClick={() => window.open("https://www.fhcm.paris/en/paris-fashion-week", "_blank")}
          />
        </div>
      )}

      {activeTab === "vocabulario" && (
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: "16px" }}>
          <div
            style={{
              backgroundColor: "var(--nex-surface-1)",
              border: "1px solid var(--nex-border-subtle)",
              borderRadius: "var(--nex-radius-md)",
              padding: "18px",
            }}
          >
            <div style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700, textTransform: "uppercase", marginBottom: "4px" }}>
              Estrutura
            </div>
            <h4 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "6px" }}>
              Prêt-à-Porter
            </h4>
            <p style={{ fontSize: "0.82rem", color: "var(--nex-text-secondary)", lineHeight: 1.45 }}>
              Moda pronta para vestir, produzida em escala padronizada com acabamentos de alta qualidade, desfilada em semanas internacionais como a de Paris.
            </p>
          </div>
          <div
            style={{
              backgroundColor: "var(--nex-surface-1)",
              border: "1px solid var(--nex-border-subtle)",
              borderRadius: "var(--nex-radius-md)",
              padding: "18px",
            }}
          >
            <div style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700, textTransform: "uppercase", marginBottom: "4px" }}>
              Calendário
            </div>
            <h4 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "6px" }}>
              Spring/Summer (SS)
            </h4>
            <p style={{ fontSize: "0.82rem", color: "var(--nex-text-secondary)", lineHeight: 1.45 }}>
              Temporada de Primavera/Verão, habitualmente apresentada nos meses de setembro/outubro do ano anterior ao do lançamento comercial.
            </p>
          </div>
        </div>
      )}

      {activeTab === "fontes" && (
        <div style={{ maxWidth: "800px", backgroundColor: "var(--nex-surface-1)", borderRadius: "var(--nex-radius-lg)", border: "1px solid var(--nex-border-subtle)", padding: "24px" }}>
          <h3 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "16px" }}>
            Fontes & Proveniência Catalogadas
          </h3>

          <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
            {sources.map((src) => (
              <div
                key={src.id}
                style={{
                  backgroundColor: "var(--nex-surface-2)",
                  borderRadius: "var(--nex-radius-md)",
                  padding: "16px",
                  border: "1px solid var(--nex-border-subtle)",
                }}
              >
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "8px" }}>
                  <div style={{ fontWeight: 600, fontSize: "0.95rem", color: "var(--nex-text-primary)" }}>
                    {src.canonicalName}
                  </div>
                  <Badge variant="tier">Tier {src.authorityTier}</Badge>
                </div>
                <div style={{ fontSize: "0.78rem", color: "var(--nex-text-muted)", marginBottom: "12px" }}>
                  Tipo: Organização Oficial • Contribuições: {src.contributions.join(", ")}
                </div>

                <div style={{ display: "flex", flexWrap: "wrap", gap: "8px" }}>
                  {src.urls.map((url, i) => (
                    <SourceLink
                      key={i}
                      label={url.split("/").slice(-2).join("/") || url}
                      url={url}
                      authorityTier={src.authorityTier}
                    />
                  ))}
                </div>
              </div>
            ))}
          </div>

          <div style={{ marginTop: "24px" }}>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-text-muted)", textTransform: "uppercase", letterSpacing: "0.06em", marginBottom: "8px" }}>
              Pipeline de Verificação
            </h4>
            <div style={{ display: "flex", flexWrap: "wrap", gap: "6px" }}>
              {julieKegelsData.pipeline.map((step, idx) => (
                <Badge key={idx} variant="canonical">
                  {step}
                </Badge>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
