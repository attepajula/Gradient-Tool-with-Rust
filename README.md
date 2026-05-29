# Gradient Tool

Extract colors from a photo — or pick them by hand — and render them into a gradient image. Built with a Rust backend and a React frontend.

**Live demo → [gradient.atte.works](https://gradient.atte.works)**

---

## How it works

Upload a photo and the API clusters its pixels in perceptually-uniform Lab color space to find the dominant colors. Those colors are then interpolated into a gradient and rendered as a JPEG you can download.

### Source photo

<p align="center">
  <img src="pp.png" width="480" alt="Source photo" />
</p>

### Extracted gradients

<p align="center">
  <img src="gradient.jpg" width="720" alt="Linear gradient extracted from photo" />
  <br/><em>Linear — straight interpolation</em>
</p>

<p align="center">
  <img src="gradient-2.jpg" width="720" alt="Radial gradient extracted from photo" />
  <br/><em>Radial — circle expanding from center</em>
</p>

---

## Features

- **Photo upload** — drag & drop a JPEG or PNG, get back 5 dominant colors
- **Manual color picker** — add, remove, and edit colors freely
- **4 gradient paradigms** — linear, diagonal, radial, reflected
- **6 warp styles** — none, ease in/out, smooth step, wave, zigzag
- **Live preview** — gradient re-renders 250 ms after any change
- **Download** — full-resolution JPEG straight from the API

---

## Stack

| Layer | Tech |
|---|---|
| API | Rust · Axum · Tokio |
| Color extraction | k-means clustering in Lab color space (`kmeans_colors`) |
| Image decode/encode | `image` crate |
| Frontend | React 18 · TypeScript · Vite |
| Hosting | Fly.io |

---

## API

### `POST /api/gradient/from-colors`

```json
{
  "colors": ["#e8534a", "#f0a500", "#4a90d9"],
  "steps": 10
}
```

### `POST /api/gradient/render`

```json
{
  "colors": ["#e8534a", "#f0a500", "#4a90d9"],
  "width": 1200,
  "height": 160,
  "paradigm": "radial",
  "warp": "ease_in_out"
}
```

Returns a `image/jpeg` binary.

**Paradigms:** `linear` · `diagonal` · `radial` · `reflected`
**Warps:** `none` · `ease_in` · `ease_out` · `ease_in_out` · `wave` · `zigzag`

### `POST /api/image/extract-colors`

Multipart upload with field `image` (JPEG or PNG, max 10 MB). Returns dominant colors and a gradient.

---

## Running locally

```bash
# Terminal 1 — API (port 3000)
cargo run -p api

# Terminal 2 — Frontend dev server (port 5173)
cd frontend && pnpm install && pnpm run dev
```

Open **http://localhost:5173**

To test the production build locally:

```bash
cd frontend && pnpm run build && cd ..
cargo run -p api
# open http://localhost:3000
```

### Shell script demo

```bash
./demo.sh photo.jpg                          # linear, no warp
./demo.sh photo.jpg -p radial -w ease_in_out # radial with easing
./demo.sh photo.jpg -p diagonal -w wave      # diagonal wave
```
