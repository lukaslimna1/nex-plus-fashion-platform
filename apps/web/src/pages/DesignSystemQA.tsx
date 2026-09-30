import React, { useState } from "react";
import {
  Button,
  IconButton,
  Badge,
  Chip,
  Tag,
  Tooltip,
  Modal,
  Drawer,
  Dropdown,
  MediaCard,
  CollectionCard,
  MaisonCard,
  CityCard,
  EventCard,
  ReviewCard,
  TermCard,
  Rail,
  SectionHeader,
  EmptyState,
  ErrorState,
  Skeleton,
  SearchInput,
  MediaCredit,
  SourceLink,
  MetadataLine,
} from "@nex-plus/ui";
import {
  Play,
  Heart,
  Share2,
  Sparkles,
  Search,
  ExternalLink,
  Layers,
  CheckCircle,
  AlertTriangle,
  Flame,
  Info,
} from "lucide-react";

export const DesignSystemQA: React.FC = () => {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);
  const [activeChip, setActiveChip] = useState("Opção 1");
  const [simulateLoading, setSimulateLoading] = useState(false);

  return (
    <div style={{ maxWidth: "1280px", margin: "0 auto", padding: "32px 24px 100px 24px" }}>
      {/* QA Header */}
      <div style={{ borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "24px", marginBottom: "40px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "8px" }}>
          <Badge variant="tier">Ferramenta Interna de QA</Badge>
          <Badge variant="canonical">Design System V2</Badge>
        </div>
        <h1 style={{ fontFamily: "var(--nex-font-display)", fontSize: "2.4rem", fontWeight: 800, marginBottom: "8px" }}>
          Design System V2 · Laboratório Visual
        </h1>
        <p style={{ color: "var(--nex-text-secondary)", fontSize: "0.95rem" }}>
          Ambiente de testes para auditoria de tokens, tipografia, superfícies, primitives, cards editoriais, feedback states e interatividade.
        </p>
      </div>

      {/* 1. Cores e Superfícies */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          1. Cores, Tons & Superfícies Dark Glass
        </h2>
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))", gap: "16px" }}>
          {[
            { label: "bg-primary", val: "#070709", text: "Fundo Geral" },
            { label: "bg-secondary", val: "#0d0d11", text: "Fundo Secundário" },
            { label: "surface-1", val: "#141419", text: "Cards Base" },
            { label: "surface-2", val: "#1e1e26", text: "Cards Elevados" },
            { label: "accent", val: "#ffffff", text: "Alto Contraste" },
            { label: "accent-gold", val: "#d4af37", text: "Ouro Couture" },
            { label: "live", val: "#ef4444", text: "Ao Vivo Transmissão" },
            { label: "success", val: "#10b981", text: "Verificado" },
            { label: "warning", val: "#f59e0b", text: "Pendente" },
            { label: "error", val: "#f43f5e", text: "Erro / Falha" },
          ].map((c) => (
            <div
              key={c.label}
              style={{
                backgroundColor: "var(--nex-surface-1)",
                border: "1px solid var(--nex-border-subtle)",
                borderRadius: "var(--nex-radius-md)",
                overflow: "hidden",
              }}
            >
              <div style={{ height: "60px", backgroundColor: c.val }} />
              <div style={{ padding: "10px" }}>
                <div style={{ fontWeight: 600, fontSize: "0.85rem", color: "var(--nex-text-primary)" }}>{c.label}</div>
                <div style={{ fontSize: "0.72rem", color: "var(--nex-text-muted)" }}>{c.val} • {c.text}</div>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* 2. Tipografia */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          2. Hierarquia Tipográfica Editorial
        </h2>
        <div style={{ display: "flex", flexDirection: "column", gap: "16px", backgroundColor: "var(--nex-surface-1)", padding: "24px", borderRadius: "var(--nex-radius-lg)", border: "1px solid var(--nex-border-subtle)" }}>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-display (Cinzel/Didot)</span>
            <div className="nex-display">PARIS FASHION WEEK®</div>
          </div>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-h1</span>
            <div className="nex-h1">Julie Kegels — Spring/Summer 2027</div>
          </div>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-h2</span>
            <div className="nex-h2">Desfile Oficial no Calendário Canônico da FHCM</div>
          </div>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-h3</span>
            <div className="nex-h3">Crítica Especializada e Leitura Estrutural dos Looks</div>
          </div>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-body</span>
            <div className="nex-body">
              NEX+ Fashion é um acervo gratuito, não oficial, educacional e cultural de Fashion Weeks e conhecimento de moda, construído para transformar conteúdo disperso em uma experiência visual moderna e acessível em português do Brasil.
            </div>
          </div>
          <div>
            <span style={{ fontSize: "0.72rem", color: "var(--nex-accent-gold)", fontWeight: 700 }}>.nex-metadata</span>
            <div className="nex-metadata">28/09/2026 • 12:30 CEST • SALLE DES FÊTES • FHCM TIER A</div>
          </div>
        </div>
      </section>

      {/* 3. Botões e Estados de Interação */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          3. Botões, IconButtons & Estados Interativos
        </h2>
        <div style={{ display: "flex", flexWrap: "wrap", gap: "16px", alignItems: "center", marginBottom: "20px" }}>
          <Button variant="primary" size="md" iconLeft={<Play size={16} fill="#000" />}>
            Primary Button
          </Button>
          <Button variant="secondary" size="md">
            Secondary Button
          </Button>
          <Button variant="outline" size="md">
            Outline Button
          </Button>
          <Button variant="ghost" size="md">
            Ghost Button
          </Button>
          <Button variant="danger" size="md">
            Danger Button
          </Button>
          <Button variant="primary" size="md" isLoading={simulateLoading} onClick={() => {
            setSimulateLoading(true);
            setTimeout(() => setSimulateLoading(false), 2000);
          }}>
            {simulateLoading ? "Carregando..." : "Testar Loading"}
          </Button>
        </div>

        <div style={{ display: "flex", gap: "12px", alignItems: "center" }}>
          <IconButton ariaLabel="Favorito" variant="primary">
            <Heart size={18} fill="#000" />
          </IconButton>
          <IconButton ariaLabel="Secundário" variant="secondary">
            <Share2 size={18} />
          </IconButton>
          <IconButton ariaLabel="Glass" variant="glass">
            <Sparkles size={18} />
          </IconButton>
          <IconButton ariaLabel="Ghost" variant="ghost">
            <Info size={18} />
          </IconButton>
        </div>
      </section>

      {/* 4. Badges, Chips e Tags */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          4. Badges, Chips Selecionáveis & Tags Editoriais
        </h2>
        <div style={{ display: "flex", flexWrap: "wrap", gap: "10px", alignItems: "center", marginBottom: "20px" }}>
          <Badge variant="live">AO VIVO</Badge>
          <Badge variant="verified">Verificado</Badge>
          <Badge variant="canonical">Canônico</Badge>
          <Badge variant="tier">Tier A</Badge>
          <Badge variant="warning">Em Revisão</Badge>
          <Badge variant="neutral">Neutro</Badge>
        </div>

        <div style={{ display: "flex", gap: "10px", flexWrap: "wrap", marginBottom: "20px" }}>
          {["Opção 1", "Opção 2", "Opção 3", "Opção 4"].map((opt) => (
            <Chip
              key={opt}
              active={activeChip === opt}
              onClick={() => setActiveChip(opt)}
            >
              {opt}
            </Chip>
          ))}
        </div>

        <div style={{ display: "flex", gap: "8px" }}>
          <Tag variant="default">Prêt-à-Porter</Tag>
          <Tag variant="gold">Alta-Costura</Tag>
          <Tag variant="subtle">Arquivos Históricos</Tag>
        </div>
      </section>

      {/* 5. Modais, Gavetas e Tooltips */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          5. Overlays, Modais, Drawers & Tooltips
        </h2>
        <div style={{ display: "flex", gap: "16px", alignItems: "center" }}>
          <Button variant="secondary" onClick={() => setIsModalOpen(true)}>
            Abrir Modal de Teste
          </Button>
          <Button variant="secondary" onClick={() => setIsDrawerOpen(true)}>
            Abrir Drawer Lateral
          </Button>
          <Tooltip content="Tooltip informativo acessível com focus e hover!">
            <Button variant="outline">Passe o mouse ou foque aqui</Button>
          </Tooltip>
        </div>

        <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Modal do Design System V2">
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.5, marginBottom: "16px" }}>
            Este modal implementa fechamento com Esc, focus containment, backdrop-blur de cinema e transição de entrada suave.
          </p>
          <div style={{ display: "flex", justifyContent: "flex-end" }}>
            <Button variant="primary" size="sm" onClick={() => setIsModalOpen(false)}>
              Entendido
            </Button>
          </div>
        </Modal>

        <Drawer isOpen={isDrawerOpen} onClose={() => setIsDrawerOpen(false)} title="Gaveta Lateral de Filtros">
          <p style={{ color: "var(--nex-text-secondary)", lineHeight: 1.5, marginBottom: "16px" }}>
            Gaveta deslizante ideal para navegação mobile, filtros avançados de busca ou painéis contextuais.
          </p>
        </Drawer>
      </section>

      {/* 6. Cards do Domínio de Moda */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          6. Cards do Domínio de Moda
        </h2>

        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(260px, 1fr))", gap: "24px" }}>
          {/* CollectionCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>CollectionCard (Verificação Pendente)</h4>
            <CollectionCard
              id="col-demo"
              name="Womenswear Spring/Summer 2027"
              maison="Julie Kegels"
              seasonLabel="Spring/Summer 2027"
              seasonCode="SS27"
              presentedOn="28/09/2026"
              format="SHOW"
              isPendingVerification={true}
            />
          </div>

          {/* CityCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>CityCard (2:3 Portrait)</h4>
            <CityCard
              id="city-demo"
              name="Paris"
              country="França"
              region="Europa"
              eventsCount={4}
              hubImportance="Global"
            />
          </div>

          {/* EventCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>EventCard (Portrait)</h4>
            <EventCard
              id="event-demo"
              name="Paris Fashion Week®"
              cityName="Paris"
              kind="Prêt-à-Porter & Haute Couture"
              organizer="FHCM"
              status="ACTIVE"
            />
          </div>

          {/* MaisonCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>MaisonCard (Streaming Inspired)</h4>
            <MaisonCard
              id="maison-demo"
              name="Julie Kegels"
              artisticDirection="Julie Kegels"
              foundedYear={2024}
              collectionsCount={1}
            />
          </div>
        </div>

        <div style={{ marginTop: "28px", display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(320px, 1fr))", gap: "20px" }}>
          {/* ReviewCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>ReviewCard (Crítica Editorial)</h4>
            <ReviewCard
              id="rev-demo"
              publication="Vogue Runway"
              criticName="Sarah Mower"
              title="A subversão das proporções clássicas"
              excerpt="Uma abordagem meticulosa sobre a alfaiataria desconstruída, resgatando técnicas manuais com rigor estético."
              maison="Julie Kegels"
              collectionName="Womenswear Spring/Summer 2027"
              seasonCode="SS27"
              publishedAt="29/09/2026"
              originalUrl="https://www.vogue.com"
            />
          </div>

          {/* TermCard */}
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-accent-gold)", marginBottom: "8px" }}>TermCard (Biblioteca)</h4>
            <TermCard
              id="term-demo"
              term="Trompe-l'œil"
              originalLanguageTerm="Trompe-l'œil (Francês)"
              language="Francês"
              definition="Técnica que emprega ilusões bidimensionais para simular dobras, bolsos ou elementos tridimensionais no corte."
              category="Técnica Têxtil"
            />
          </div>
        </div>
      </section>

      {/* 7. Feedback & Skeletons */}
      <section style={{ marginBottom: "50px" }}>
        <h2 style={{ fontSize: "1.3rem", fontWeight: 700, marginBottom: "16px", borderBottom: "1px solid var(--nex-border-subtle)", paddingBottom: "8px" }}>
          7. Skeletons, Empty States & Error States
        </h2>
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(300px, 1fr))", gap: "20px" }}>
          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-text-muted)", marginBottom: "8px" }}>Shimmer Skeletons</h4>
            <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
              <Skeleton height="36px" width="60%" />
              <Skeleton height="18px" width="90%" />
              <Skeleton height="18px" width="75%" />
              <Skeleton height="140px" width="100%" borderRadius="var(--nex-radius-lg)" />
            </div>
          </div>

          <div>
            <h4 style={{ fontSize: "0.85rem", color: "var(--nex-text-muted)", marginBottom: "8px" }}>ErrorState com Retry</h4>
            <ErrorState
              title="Falha na sincronização remota"
              message="Não foi possível obter dados da FHCM. O cache local está disponível."
              onRetry={() => alert("Tentando recarregar...")}
            />
          </div>
        </div>
      </section>
    </div>
  );
};
