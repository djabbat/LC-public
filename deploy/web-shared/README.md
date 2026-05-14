<!-- AUTO-TRANSLATED from README.md via DeepSeek 2026-05-13. Source language: russian. Original (README.md) is canonical; re-run scripts/translate_core_files.py after edits. -->

# deploy/web-shared/ — cross-subdomain assets

Server assets that are injected via nginx `sub_filter` or
included by the OJS theme on ALL longevity.ge subdomains.

## Files

- **`eco-inject.js`** — common sticky header (LongevityCommon brand +
 ecosystem navigation) + dark/light toggle + dark theme
 CSS overrides for PKP/OJS, Tailwind, custom styles.

 Loaded on each subdomain from `https://longevity.ge/eco-inject.js`.
 Cookie `lc_theme` on `.longevity.ge` synchronizes the theme across
 subdomains.

 The theme, navigation, and dark mode constitute the sole mechanism; do not duplicate
 in OJS / Phoenix / Vite themes.

## Deploy to server

Canonical path on the server: `/home/jaba/web/longevity/eco-inject.js`
(symlinks/copies in `/home/jaba/web/ngo/`, etc. for legacy paths).

```bash
scp deploy/web-shared/eco-inject.js \
 server:/home/jaba/web/longevity/eco-inject.js
ssh server "sudo cp /home/jaba/web/longevity/eco-inject.js \
 /home/jaba/web/ngo/eco-inject.js"
```

After deployment — bump `?v=` query string in OJS theme / nginx snippet,
otherwise CF and browsers will cache the old version.

## Bump cache version

After any change to the JS:

```bash
ssh server 'cd /home/jaba/web/longevity && \
 grep -rl "eco-inject.js?v=" plugins/themes/ | \
 xargs sed -i "s/eco-inject.js?v=[0-9]*/eco-inject.js?v=$(date +%s | tail -c 4)/g"'