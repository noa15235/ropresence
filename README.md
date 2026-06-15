# RoPresence

**Rich Presence Discord pour Roblox** — une application de bureau légère (Tauri 2 +
React + Rust) qui détecte automatiquement le jeu Roblox que vous lancez et l'affiche
sur votre profil Discord, via la **Rich Presence officielle**.

- 🪶 Légère — backend Rust, binaire compact, conso CPU ≈ 0 % au repos.
- 🎨 Belle — design glassmorphism sombre/clair, animations Framer Motion.
- 🔒 Conforme & sûre — **aucun token Discord, aucun cookie Roblox, aucun self-bot**.
  Uniquement le RPC officiel Discord + les API publiques Roblox + les logs locaux.

---

## Comment ça marche

1. **Détection Roblox** — l'app lit les logs locaux de Roblox
   (`%LOCALAPPDATA%\Roblox\logs`) et la liste des processus pour savoir à quel jeu
   vous jouez (placeId / universeId / jobId). Elle n'utilise jamais votre
   cookie/compte Roblox.
2. **Infos du jeu** — à partir du placeId, elle interroge les API publiques Roblox
   (`games`, `thumbnails`, `apis`) pour le nom de l'expérience, le créateur et
   l'icône.
3. **Discord** — elle se connecte au client Discord déjà ouvert via le socket IPC
   local (`discord-ipc-0`) et envoie une *Activity*. C'est l'usage prévu et
   conforme aux ToS.

---

## Prérequis

- **Windows 10/11**
- **Node.js** 18+ et **pnpm** (`npm i -g pnpm`)
- **Rust** (toolchain MSVC) — https://rustup.rs
- **Build Tools C++ de Visual Studio** (workload « Desktop development with C++ » +
  Windows SDK) et **WebView2** (préinstallé sur Windows 11).

---

## Créer votre application Discord (obligatoire)

La Rich Presence nécessite un **Client ID** (Application ID) que vous créez en 2 min :

1. Ouvrez le **Discord Developer Portal** :
   https://discord.com/developers/applications
2. Cliquez sur **New Application**, donnez-lui un nom — ce nom apparaîtra comme
   « joue à *…* » sur votre profil.
3. Dans **General Information**, copiez l'**Application ID** (un nombre de 17–19
   chiffres).
4. Collez-le dans l'assistant de configuration de RoPresence au premier lancement.

### (Optionnel) Images personnalisées — Art Assets

Discord n'affiche que des images **déjà uploadées dans votre application** ou des
**URL d'image**. Pour utiliser une clé d'asset :

1. Dans le portail, allez dans **Rich Presence → Art Assets**.
2. **Add Image(s)**, uploadez votre image et donnez-lui un nom (la *clé*),
   par ex. `roblox`.
3. Dans RoPresence (onglet *Présence*), choisissez la source **« Clé d'asset
   Discord »** et saisissez cette clé.

> Par défaut, la grande image est en mode **« Icône du jeu (auto) »** : RoPresence
> envoie directement l'URL de l'icône Roblox du jeu en cours, ce qui évite tout
> upload. Si votre client Discord n'affiche pas les images par URL, basculez sur une
> **clé d'asset**.

---

## Installation & lancement

```bash
pnpm install          # dépendances front
pnpm tauri dev        # lance l'app en mode développement
```

### Build de production

```bash
pnpm tauri build      # génère l'installeur (NSIS) dans src-tauri/target/release/bundle
```

> Astuce : si `cargo`/`tauri` ne sont pas trouvés, vérifiez que
> `%USERPROFILE%\.cargo\bin` est dans votre `PATH` (rouvrez le terminal après
> l'installation de Rust).

---

## Variables dynamiques

Utilisables dans les champs *Détails*, *État* et *Texte au survol* :

| Variable        | Valeur                                  |
| --------------- | --------------------------------------- |
| `{game}`        | Nom de l'expérience                     |
| `{creator}`     | Nom du créateur                         |
| `{username}`    | Votre pseudo Roblox (compte actif)      |
| `{placeId}`     | Identifiant de la place                 |
| `{universeId}`  | Identifiant de l'univers                |
| `{players}`     | Nombre de joueurs (si disponible)       |
| `{jobId}`       | Identifiant de l'instance               |
| `{time}`        | Temps écoulé dans la session            |

---

## Fonctionnalités

**Cœur (MVP)** : connexion RPC officielle, assistant de configuration, détection
auto du jeu, infos via API Roblox, grande/petite image, textes personnalisables avec
variables, minuteur de session, master switch, indicateurs Discord/Roblox en temps
réel, toggles par fonctionnalité, persistance auto, system tray + fermer-vers-tray,
édition live, aperçu Discord live, reconnexion auto, mode privé.

**En plus** : boutons custom + bouton auto « Voir l'expérience », avatar Roblox,
thème clair + couleur d'accent, profils multiples, présence statique, lancement au
démarrage, démarrage minimisé, notifications natives, multi-comptes, détection
Studio, import/export JSON, raccourci global, FR/EN, panneau de logs, compteur de
joueurs (party).

---

## Architecture

```
src-tauri/src/
  main.rs            # setup app + tray + close-to-tray + worker
  state.rs           # état partagé (config + runtime) + signaux du worker
  commands.rs        # commands Tauri exposées au front
  tray.rs            # icône + menu system tray
  config/            # modèle de config + persistance (tauri-plugin-store)
  discord/           # connexion IPC + envoi Activity + backoff de reconnexion
  roblox/            # process watch + parsing logs + API publiques
  presence/          # worker + builder Activity + résolution des variables
src/
  pages/             # Setup, Presence, Roblox, Buttons, Profiles, Settings
  components/        # Sidebar, StatusBar, DiscordPreview, Toggle, …
  store/             # Zustand (config + runtime)
  i18n/              # fr.json / en.json
  styles/            # theme.css (tokens) + global.css
```

---

## Confidentialité & conformité

- Aucun **token Discord**, aucune lecture du compte/amis Discord.
- Aucun **cookie/token Roblox**, aucun scraping authentifié.
- Aucun **self-bot**. Uniquement : RPC officiel Discord + API publiques Roblox +
  logs locaux en lecture seule.
