# Portfolio

Site portfolio one-page : frontend Angular avec SSR et API Rust. Design sombre type terminal
(JetBrains Mono + Fraunces, accent vert), issu d'une maquette Figma Make.

## Structure

- `frontend/` : Angular 22 + SSR (`@angular/ssr`), one-page rendue côté serveur
- `api/` : API Axum : `GET /api/projects`, `GET /api/csrf`, `POST /api/contact`, `GET /health`
- `ferrflow.json` : versioning FerrFlow par package (`frontend@vX.Y.Z`, `api@vX.Y.Z`)

## Développement

```sh
cd api && cargo run
cd frontend && pnpm start
```

Le dev server Angular proxifie `/api` vers `http://localhost:8080` (`proxy.conf.json`).
En production, le serveur SSR proxifie `/api` vers `API_URL` (défaut `http://localhost:8080`).

## Contenu

Les projets affichés viennent de `api/assets/projects.json` (slug, catégorie, statut, année,
description, stack, liens, image optionnelle). Le reste du contenu (hero, stack, contact) vit
dans les composants de `frontend/src/app/pages/home/sections/`.

## SEO

- SSR runtime (RenderMode.Server) : la page arrive complète, projets inclus
- Balises meta, Open Graph, canonical et JSON-LD gérés par `frontend/src/app/core/seo.ts`
- `robots.txt` et `sitemap.xml` servis depuis `frontend/public/`
- Le domaine du site est centralisé dans `frontend/src/app/core/site.ts`

## API : configuration

| Variable | Rôle |
|---|---|
| `PORT` | Port d'écoute (défaut `8080`) |
| `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD` | Relais SMTP pour `POST /api/contact` |
| `CONTACT_FROM`, `CONTACT_TO` | Expéditeur et destinataire des messages |

Le formulaire de contact est protégé par un token CSRF en double soumission : `GET /api/csrf`
pose un cookie HttpOnly SameSite=Strict et renvoie le token, que `POST /api/contact` exige dans
l'en-tête `X-Csrf-Token`. Sans configuration SMTP complète, l'endpoint répond `503`.

## CI / Release

- `ci.yml` : checks frontend (tests + build) et api (fmt, clippy, tests), puis release FerrFlow sur `main`
- FerrFlow calcule les versions depuis les commits conventionnels et publie les tags `frontend@vX.Y.Z` / `api@vX.Y.Z`
- `docker.yml` : à la publication d'une release, build et push de l'image du package concerné vers GHCR (`<version>`, `<major.minor>`, `latest`)
