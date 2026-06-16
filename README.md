<div align="center">

# 🎮 RoPresence

### Rich Presence Discord pour Roblox — légère, élégante, et sans risque pour ton compte.

[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB?logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=black)](https://react.dev)
[![TypeScript](https://img.shields.io/badge/TypeScript-strict-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![Windows](https://img.shields.io/badge/Windows-10%20%2F%2011-0078D6?logo=windows&logoColor=white)](#)

</div>

---

**RoPresence** détecte automatiquement le jeu Roblox que tu lances et l'affiche sur ton profil Discord : **icône du jeu, nom, créateur, minuteur** et boutons cliquables. Le tout via la **Rich Presence officielle de Discord** — **aucun token, aucun bot, aucun cookie**. Zéro risque pour ton compte Roblox ou Discord.

## ✨ Fonctionnalités

- 🎯 **Détection automatique** du jeu Roblox (lecture seule des logs locaux).
- 🖼️ **Icône du jeu**, nom et créateur affichés en direct sur Discord.
- ⏱️ **Trois minuteurs** : temps sur le jeu en cours (remis à zéro à chaque jeu), session Roblox totale, et temps de jeu du jour (remis à zéro à minuit).
- 🔘 **Boutons automatiques** : *Rejoindre* + *Mon profil* Roblox.
- 👤 **Connexion du compte** Roblox en un clic (avatar + profil), avec confirmation.
- 🎨 **Interface glassmorphism** : thème sombre/clair, couleur d'accent, FR/EN.
- 🧩 **Modèles** de présence en 1 clic (Détaillé, Simple, Streamer, Discret) + personnalisation complète.
- 🪶 **Légère** : se réduit dans le system tray, ~0 % CPU au repos, RAM minimale.
- 🔒 **Sûre & conforme** : RPC Discord officiel + API publiques Roblox + logs locaux. Rien d'autre ne quitte ta machine.

## 🚀 Installation

1. Va dans **[Releases](https://github.com/noa15235/ropresence/releases)** et télécharge la dernière version.
2. Décompresse et lance **`ropresence.exe`**.
3. Ça fonctionne immédiatement (application Roblox Discord intégrée).
   Pour ton propre nom/branding, crée une app Discord et colle son *Application ID* dans **Réglages → Discord** (un tuto est intégré dans l'app).

> Windows 10/11. WebView2 (préinstallé sur Windows 11) est requis.

## 🛠️ Build depuis les sources

Prérequis : **Node.js 18+**, **pnpm**, **Rust** (https://rustup.rs) et un linker C++.

```bash
pnpm install
pnpm tauri dev      # développement
pnpm tauri build    # build de production
```

## 🔐 Confidentialité

| | |
|---|---|
| Token / bot / self-bot Discord | ❌ jamais |
| Cookie / token de compte Roblox | ❌ jamais |
| Logs Roblox locaux | ✅ lecture seule |
| API publiques Roblox (nom, icône, avatar) | ✅ non authentifiées |
| Données envoyées à des tiers | ❌ aucune |

## 🧱 Stack

Tauri 2 · Rust · React 18 · TypeScript · Vite · Zustand · Framer Motion · lucide-react

## 📂 Architecture

```
src-tauri/src/
  main.rs        setup app + tray + worker
  state.rs       état partagé (config + runtime)
  commands.rs    commandes exposées au front
  config/        modèle de config + persistance
  discord/       connexion IPC + envoi Activity + reconnexion
  roblox/        détection process + parsing logs + API publiques
  presence/      worker + construction de l'Activity + variables
src/
  pages/         Presence, Roblox, Profils, Réglages
  components/    Sidebar, StatusBar, DiscordPreview, SessionStats, …
  store/         Zustand (config + runtime)
  i18n/          fr.json / en.json
  styles/        theme.css + global.css
```

## 🔣 Variables dynamiques

`{game}` · `{creator}` · `{username}` · `{userId}` · `{placeId}` · `{universeId}` · `{players}` · `{jobId}` · `{time}`

---

<div align="center">

Fait avec ❤️ pour la communauté Roblox · **RoPresence**

</div>
