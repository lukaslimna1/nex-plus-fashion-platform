import React, { useState } from "react";
import {
  Calendar,
  Clock,
  MapPin,
  ExternalLink,
  ShieldCheck,
  BookOpen,
  Film,
  Camera,
  Layers,
  Sparkles,
  Edit3,
  Bookmark,
  Share2,
  AlertCircle,
  Eye,
} from "lucide-react";
import {
  Badge,
  Button,
  IconButton,
  Chip,
  EmptyState,
  SourceLink,
  MetadataLine,
  MediaCredit,
} from "@nex-plus/ui";

// Importando o contrato real da vertical slice produzido e atualizado pelo Codex
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
  const [selectedLook, setSelectedLook] = useState<any>(null);

  const handleSaveNote = () => {
    localStorage.setItem("caderno_julie_kegels_ss27", cadernoNote);
    setIsSaved(true);
    setTimeout(() => setIsSaved(false), 2000);
  };

  const { collection, schedule, sources, media } = julieKegelsData;

  // Filtragem dos assets reais do Codex
  const imageAssets = media.assets.filter((a) => a.assetKind === "IMAGE");
  const videoAssets = media.assets.filter((a) => a.assetKind === "VIDEO" || a.embedUrl);

  const isMediaReady = media.status === "MEDIA_READY_WITH_RIGHTS_GUARD";

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
          marginBottom: "32px",
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
            <Badge variant="neutral">
              Formato: {schedule.format === "SHOW" ? "Desfile Oficial" : schedule.format}
            </Badge>
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

      {/* Rights Guard & Provenance Notification Banner */}
      {isMediaReady && (
        <div
          style={{
            backgroundColor: "rgba(16, 185, 129, 0.06)",
            border: "1px solid rgba(16, 185, 129, 0.25)",
            borderRadius: "var(--nex-radius-lg)",
            padding: "16px 20px",
            marginBottom: "28px",
            display: "flex",
            alignItems: "flex-start",
            gap: "14px",
          }}
        >
          <ShieldCheck size={22} style={{ color: "var(--nex-success)", flexShrink: 0, marginTop: "2px" }} />
          <div>
            <div style={{ fontSize: "0.88rem", fontWeight: 600, color: "var(--nex-text-primary)", marginBottom: "2px" }}>
              Mídia Homologada com Proteção de Direitos (MEDIA_READY_WITH_RIGHTS_GUARD)
            </div>
            <p style={{ fontSize: "0.82rem", color: "var(--nex-text-secondary)", lineHeight: 1.45 }}>
              Mídia remota de terceiros agregada via URL/embed, com atribuição explícita de fotógrafo, veículo e página original. Downloads desabilitados conforme a política das fontes oficiais.
            </p>
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
          { id: "looks", label: `Looks do Desfile (${imageAssets.length})` },
          { id: "leitura", label: "Minha Leitura / Caderno" },
          { id: "critica", label: "Crítica Especializada (1)" },
          { id: "videos", label: `Vídeos & Cobertura (${videoAssets.length})` },
          { id: "vocabulario", label: "Vocabulário Relacionado" },
          { id: "fontes", label: `Fontes & Proveniência (${sources.length})` },
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

      {/* Tab 1: Looks do Desfile (com Mídia REAL e Créditos) */}
      {activeTab === "looks" && (
        <div>
          <div style={{ marginBottom: "20px", display: "flex", justifyContent: "space-between", alignItems: "baseline" }}>
            <div>
              <h3 style={{ fontSize: "1.2rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
                Looks da Coleção · Cobertura Fotográfica
              </h3>
              <p style={{ fontSize: "0.82rem", color: "var(--nex-text-muted)" }}>
                Fotografia oficial da passarela capturada durante a apresentação na Paris Fashion Week®.
              </p>
            </div>
            <Badge variant="canonical">1 look homologado</Badge>
          </div>

          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill, minmax(280px, 340px))",
              gap: "24px",
            }}
          >
            {imageAssets.map((asset) => (
              <div
                key={asset.id}
                style={{
                  backgroundColor: "var(--nex-surface-1)",
                  borderRadius: "var(--nex-radius-lg)",
                  border: "1px solid var(--nex-border-subtle)",
                  overflow: "hidden",
                  display: "flex",
                  flexDirection: "column",
                  transition: "all var(--nex-transition-normal)",
                }}
                className="nex-glass-hover"
              >
                {/* Look Photo Container */}
                <div
                  style={{
                    position: "relative",
                    width: "100%",
                    aspectRatio: "2/3",
                    backgroundColor: "var(--nex-surface-2)",
                    overflow: "hidden",
                  }}
                >
                  <img
                    src={asset.remoteUrl || asset.thumbnailUrl}
                    alt={asset.id.includes("look-01") ? "Julie Kegels SS27 — Look 01 (Passarela)" : "Julie Kegels SS27 Look"}
                    loading="lazy"
                    style={{
                      width: "100%",
                      height: "100%",
                      objectFit: "cover",
                      transition: "transform var(--nex-transition-cinematic)",
                    }}
                    className="nex-card-image"
                  />

                  {/* Badges on Top */}
                  <div
                    style={{
                      position: "absolute",
                      top: "10px",
                      left: "10px",
                      display: "flex",
                      gap: "6px",
                      zIndex: 2,
                    }}
                  >
                    <Badge variant="canonical" size="sm">
                      Look 01
                    </Badge>
                    <Badge variant="neutral" size="sm">
                      Passarela
                    </Badge>
                  </div>
                </div>

                {/* Attribution & Provenance Footer */}
                <div style={{ padding: "14px 16px", display: "flex", flexDirection: "column", gap: "8px" }}>
                  <div style={{ fontWeight: 600, fontSize: "0.95rem", color: "var(--nex-text-primary)" }}>
                    {asset.id.includes("look-01") ? "Julie Kegels SS27 — Look 01 (Passarela)" : "Julie Kegels SS27 Look"}
                  </div>

                  <div style={{ display: "flex", flexDirection: "column", gap: "4px", fontSize: "0.78rem" }}>
                    <div style={{ display: "flex", alignItems: "center", gap: "5px", color: "var(--nex-text-secondary)" }}>
                      <Camera size={13} style={{ color: "var(--nex-accent-gold)" }} />
                      <span>Fotógrafo:</span>
                      <strong style={{ color: "var(--nex-text-primary)", fontWeight: 600 }}>
                        {asset.photographer}
                      </strong>
                    </div>

                    <div style={{ display: "flex", alignItems: "center", gap: "5px", color: "var(--nex-text-muted)" }}>
                      <BookOpen size={13} />
                      <span>Fonte:</span>
                      <span style={{ color: "var(--nex-text-secondary)" }}>
                        {asset.provider} (Launchmetrics)
                      </span>
                    </div>

                    <div style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)", fontStyle: "italic", marginTop: "2px" }}>
                      {asset.creditLine}
                    </div>
                  </div>

                  {/* Action Link: Ver original */}
                  <div
                    style={{
                      marginTop: "6px",
                      paddingTop: "10px",
                      borderTop: "1px solid var(--nex-border-subtle)",
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "center",
                    }}
                  >
                    <span style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)" }}>
                      Download desabilitado
                    </span>
                    <a
                      href={asset.sourcePageUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      style={{
                        display: "inline-flex",
                        alignItems: "center",
                        gap: "4px",
                        color: "var(--nex-text-primary)",
                        fontSize: "0.8rem",
                        fontWeight: 600,
                        textDecoration: "underline",
                        textUnderlineOffset: "3px",
                      }}
                    >
                      <span>Ver original na FHCM</span>
                      <ExternalLink size={12} />
                    </a>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Tab 2: Minha Leitura / Caderno Pessoal */}
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

      {/* Tab 3: Crítica Especializada */}
      {activeTab === "critica" && (
        <div style={{ maxWidth: "800px" }}>
          <div
            style={{
              backgroundColor: "var(--nex-surface-1)",
              border: "1px solid var(--nex-border-subtle)",
              borderRadius: "var(--nex-radius-lg)",
              padding: "24px",
              marginBottom: "24px",
            }}
          >
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "12px" }}>
              <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                <BookOpen size={16} style={{ color: "var(--nex-accent-gold)" }} />
                <span style={{ fontWeight: 700, fontSize: "0.85rem", textTransform: "uppercase", letterSpacing: "0.06em" }}>
                  nss magazine
                </span>
                <Badge variant="canonical" size="sm">
                  Tier B
                </Badge>
              </div>
              <span style={{ fontSize: "0.78rem", color: "var(--nex-text-muted)" }}>29/09/2026</span>
            </div>

            <h4
              style={{
                fontFamily: "var(--nex-font-display)",
                fontSize: "1.25rem",
                fontWeight: 700,
                color: "var(--nex-text-primary)",
                marginBottom: "10px",
              }}
            >
              Julie Kegels runway show — Spring/Summer 2027
            </h4>

            <p style={{ fontSize: "0.9rem", color: "var(--nex-text-secondary)", lineHeight: 1.6, marginBottom: "16px" }}>
              Resenha crítica da publicação especializada nss magazine cobrindo a estreia e evolução estética da designer Julie Kegels no circuito oficial da semana de moda parisiense.
            </p>

            <a
              href="https://www.nssmag.com/en/fashion/47057/julie-kegels-runway-show-spring-summer-2027-paris-fashion-week"
              target="_blank"
              rel="noopener noreferrer"
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "6px",
                color: "var(--nex-text-primary)",
                fontSize: "0.84rem",
                fontWeight: 600,
                textDecoration: "underline",
                textUnderlineOffset: "3px",
              }}
            >
              <span>Ler artigo completo no nss magazine</span>
              <ExternalLink size={13} />
            </a>
          </div>
        </div>
      )}

      {/* Tab 4: Vídeos & Cobertura (Reel NSS como COBERTURA, não desfile completo!) */}
      {activeTab === "videos" && (
        <div style={{ maxWidth: "800px" }}>
          <div style={{ marginBottom: "20px" }}>
            <h3 style={{ fontSize: "1.2rem", fontWeight: 700, color: "var(--nex-text-primary)", marginBottom: "4px" }}>
              Vídeos & Cobertura Audiovisual
            </h3>
            <p style={{ fontSize: "0.84rem", color: "var(--nex-text-muted)" }}>
              Registros audiovisuais verificados e coberturas em formato reel homologadas pelo pipeline.
            </p>
          </div>

          {videoAssets.map((asset) => (
            <div
              key={asset.id}
              style={{
                backgroundColor: "var(--nex-surface-1)",
                borderRadius: "var(--nex-radius-lg)",
                border: "1px solid var(--nex-border-subtle)",
                padding: "24px",
                display: "flex",
                flexDirection: "column",
                gap: "16px",
              }}
            >
              {/* Header com distinção explícita de Reel vs Desfile Completo */}
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", flexWrap: "wrap", gap: "10px" }}>
                <div>
                  <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
                    <Badge variant="tier" size="sm">
                      Cobertura em Reel
                    </Badge>
                    <span style={{ fontSize: "0.78rem", color: "var(--nex-text-muted)" }}>
                      Instagram • @{asset.creator}
                    </span>
                  </div>
                  <h4 style={{ fontSize: "1.1rem", fontWeight: 700, color: "var(--nex-text-primary)" }}>
                    {asset.id.includes("reel") ? "Julie Kegels SS27 — Reel de Cobertura (@nssfrance)" : asset.id}
                  </h4>
                </div>

                <a
                  href={asset.sourcePageUrl}
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
                  <span>Matéria no nss magazine</span>
                  <ExternalLink size={12} />
                </a>
              </div>

              {/* Explicação de Proveniência Canônica */}
              <div
                style={{
                  backgroundColor: "rgba(255, 255, 255, 0.04)",
                  border: "1px solid var(--nex-border-subtle)",
                  borderRadius: "var(--nex-radius-md)",
                  padding: "12px 14px",
                  fontSize: "0.8rem",
                  color: "var(--nex-text-secondary)",
                  lineHeight: 1.45,
                  display: "flex",
                  alignItems: "center",
                  gap: "10px",
                }}
              >
                <AlertCircle size={16} style={{ color: "var(--nex-accent-gold)", flexShrink: 0 }} />
                <span>
                  <strong>Nota Editorial:</strong> Este reel do Instagram registra momentos da apresentação na Paris Fashion Week®. Não se trata da transmissão integral ou do vídeo master do desfile.
                </span>
              </div>

              {/* Responsive Embed do Reel do Instagram */}
              <div
                style={{
                  width: "100%",
                  maxWidth: "400px",
                  margin: "0 auto",
                  borderRadius: "var(--nex-radius-md)",
                  overflow: "hidden",
                  border: "1px solid var(--nex-border-strong)",
                  backgroundColor: "#000",
                }}
              >
                <iframe
                  src={asset.embedUrl}
                  width="100%"
                  height="480"
                  style={{ border: "none", overflow: "hidden" }}
                  scrolling="no"
                  allowTransparency={true}
                  allow="encrypted-media"
                  title="Reel nss magazine da apresentação Julie Kegels SS27"
                />
              </div>

              <div style={{ fontSize: "0.75rem", color: "var(--nex-text-muted)", textAlign: "center" }}>
                {asset.creditLine}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Tab 5: Vocabulário Relacionado */}
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

      {/* Tab 6: Fontes & Proveniência */}
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
                  Tipo: {src.type} • Contribuições: {src.contributions.join(", ")}
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
              Pipeline de Verificação Canônica
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
