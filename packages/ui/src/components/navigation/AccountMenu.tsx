import React from "react";
import { User, Settings, Info, CloudCheck, RefreshCw, LogOut, Bookmark, ExternalLink } from "lucide-react";
import { Dropdown } from "../primitives/Dropdown";
import type { MenuItem } from "../primitives/Dropdown";

export interface AccountMenuProps {
  isLoggedIn?: boolean | undefined;
  userName?: string | undefined;
  userAvatar?: string | undefined;
  isDesktop?: boolean | undefined;
  onOpenAbout?: (() => void) | undefined;
  onOpenSettings?: (() => void) | undefined;
  onOpenFavorites?: (() => void) | undefined;
  onLogin?: (() => void) | undefined;
  onLogout?: (() => void) | undefined;
  onReindexLocal?: (() => void) | undefined;
}

export const AccountMenu: React.FC<AccountMenuProps> = ({
  isLoggedIn = false,
  userName = "Lucas Lima",
  userAvatar,
  isDesktop = false,
  onOpenAbout,
  onOpenSettings,
  onOpenFavorites,
  onLogin,
  onLogout,
  onReindexLocal,
}) => {
  const menuItems: MenuItem[] = [
    {
      id: "profile",
      label: isLoggedIn ? userName : "Entrar com Google",
      icon: <User size={15} />,
      onClick: isLoggedIn ? onOpenSettings : onLogin,
    },
    {
      id: "sync",
      label: "Sincronização: Ativa",
      icon: <CloudCheck size={15} style={{ color: "var(--nex-success)" }} />,
      disabled: !isLoggedIn,
    },
    {
      id: "favorites",
      label: "Meus Favoritos",
      icon: <Bookmark size={15} />,
      onClick: onOpenFavorites,
    },
    {
      id: "div-1",
      label: "",
      divider: true,
    },
    // Somente no desktop aparece 'Reindexar acervo local'
    ...(isDesktop
      ? [
          {
            id: "reindex",
            label: "Reindexar acervo local",
            icon: <RefreshCw size={15} />,
            onClick: onReindexLocal,
          },
        ]
      : []),
    {
      id: "about",
      label: "Sobre o NEX+ Fashion",
      icon: <Info size={15} />,
      onClick: onOpenAbout,
    },
    {
      id: "settings",
      label: "Preferências",
      icon: <Settings size={15} />,
      onClick: onOpenSettings,
    },
    ...(isLoggedIn
      ? [
          {
            id: "div-2",
            label: "",
            divider: true,
          },
          {
            id: "logout",
            label: "Sair da conta",
            icon: <LogOut size={15} />,
            danger: true,
            onClick: onLogout,
          },
        ]
      : []),
  ];

  return (
    <Dropdown
      trigger={
        <div
          title={isLoggedIn ? userName : "Menu da Conta"}
          style={{
            width: "36px",
            height: "36px",
            borderRadius: "50%",
            backgroundColor: "var(--nex-surface-2)",
            border: "1px solid var(--nex-border-strong)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            color: "var(--nex-text-primary)",
            cursor: "pointer",
            overflow: "hidden",
            boxShadow: "var(--nex-shadow-sm)",
          }}
          className="nex-avatar-btn"
        >
          {userAvatar ? (
            <img src={userAvatar} alt={userName} style={{ width: "100%", height: "100%", objectFit: "cover" }} />
          ) : (
            <User size={18} />
          )}
        </div>
      }
      items={menuItems}
      align="right"
    />
  );
};
