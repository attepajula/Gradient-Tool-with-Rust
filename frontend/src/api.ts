const BASE = '/api'

export interface ExtractColorsResponse {
  dominant_colors: string[]
  gradient: { stops: { position: number; hex: string }[] }
}

export interface RenderParams {
  stops: { hex: string; position: number }[]
  width: number
  height: number
  quality: number
  paradigm: string
  warp: string
  noise: number
}

export async function extractColors(file: File): Promise<ExtractColorsResponse> {
  const form = new FormData()
  form.append('image', file)
  const res = await fetch(`${BASE}/image/extract-colors`, { method: 'POST', body: form })
  if (!res.ok) {
    const body = await res.text()
    throw new Error(`Color extraction failed: ${body}`)
  }
  return res.json()
}

export async function renderGradient(params: RenderParams): Promise<Blob> {
  const res = await fetch(`${BASE}/gradient/render`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  })
  if (!res.ok) {
    const body = await res.text()
    throw new Error(`Render failed: ${body}`)
  }
  return res.blob()
}
