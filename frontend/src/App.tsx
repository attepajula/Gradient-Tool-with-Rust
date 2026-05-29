import { useState, useEffect, useCallback, useRef } from 'react'
import { extractColors, renderGradient } from './api'
import ImageUpload from './components/ImageUpload'
import ColorPalette from './components/ColorPalette'
import GradientControls from './components/GradientControls'
import GradientPreview from './components/GradientPreview'

export type Paradigm = 'linear' | 'diagonal' | 'radial' | 'reflected' | 'free'

export interface Stop {
  id: string
  hex: string
  position: number  // 0–1 scalar, used by non-free paradigms
  x: number         // 0–1 normalized image coord, used by free paradigm
  y: number
}

let nextId = 1
export function makeStop(hex: string, position: number, x?: number, y?: number): Stop {
  return { id: String(nextId++), hex, position, x: x ?? position, y: y ?? 0.5 }
}

export function stopToXY(stop: Stop, paradigm: Paradigm): { x: number; y: number } {
  const t = stop.position
  switch (paradigm) {
    case 'linear':    return { x: t, y: 0.5 }
    case 'diagonal':  return { x: t, y: t }
    case 'radial':    return { x: 0.5 + t * 0.5, y: 0.5 }
    case 'reflected': return { x: 0.5 + (1 - t) * 0.5, y: 0.5 }
    case 'free':      return { x: stop.x, y: stop.y }
  }
}

const DEFAULT_STOPS: Stop[] = [
  makeStop('#e8534a', 0),
  makeStop('#f0a500', 0.5),
  makeStop('#4a90d9', 1),
]

export default function App() {
  const [stops, setStops] = useState<Stop[]>(DEFAULT_STOPS)
  const [paradigm, setParadigm] = useState<Paradigm>('linear')
  const [width, setWidth] = useState(1200)
  const [height, setHeight] = useState(200)
  const [noise, setNoise] = useState(0)
  const [gradientUrl, setGradientUrl] = useState<string | null>(null)
  const [rendering, setRendering] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const prevUrlRef = useRef<string | null>(null)
  const paradigmRef = useRef(paradigm)
  paradigmRef.current = paradigm

  const handleParadigmChange = (next: Paradigm) => {
    const prev = paradigmRef.current
    setStops(s => s.map(stop => {
      if (next === 'free') {
        // snapshot current visual position into x, y
        const { x, y } = stopToXY(stop, prev)
        return { ...stop, x, y }
      }
      return stop
    }))
    setParadigm(next)
  }

  const render = useCallback(async () => {
    if (stops.length === 0) return
    setRendering(true)
    setError(null)
    try {
      const blob = await renderGradient({
        stops: stops.map(s => ({ hex: s.hex, position: s.position, x: s.x, y: s.y })),
        width,
        height,
        paradigm,
        warp: 'none',
        noise,
        quality: 92,
      })
      const url = URL.createObjectURL(blob)
      if (prevUrlRef.current) URL.revokeObjectURL(prevUrlRef.current)
      prevUrlRef.current = url
      setGradientUrl(url)
    } catch (e) {
      setError(String(e))
    } finally {
      setRendering(false)
    }
  }, [stops, paradigm, width, height, noise])

  useEffect(() => {
    const t = setTimeout(render, 250)
    return () => clearTimeout(t)
  }, [render])

  const handleImageUpload = async (file: File) => {
    setUploading(true)
    setError(null)
    try {
      const { dominant_colors } = await extractColors(file)
      const n = dominant_colors.length
      setStops(dominant_colors.map((hex, i) => {
        const position = n === 1 ? 0 : i / (n - 1)
        return makeStop(hex, position)
      }))
    } catch (e) {
      setError(String(e))
    } finally {
      setUploading(false)
    }
  }

  return (
    <div className="app">
      <header className="header">
        <div className="header-inner">
          <h1>Gradient Tool</h1>
          <p>Upload a photo to extract colors, or pick them manually</p>
        </div>
      </header>

      <main className="main">
        <aside className="sidebar">
          <ImageUpload onUpload={handleImageUpload} loading={uploading} />
          <ColorPalette stops={stops} onChange={setStops} />
        </aside>

        <section className="content">
          <GradientControls
            paradigm={paradigm}
            width={width}
            height={height}
            noise={noise}
            onParadigmChange={handleParadigmChange}
            onWidthChange={setWidth}
            onHeightChange={setHeight}
            onNoiseChange={setNoise}
            onStopsChange={setStops}
          />
          <GradientPreview
            url={gradientUrl}
            loading={rendering}
            error={error}
            stops={stops}
            paradigm={paradigm}
            onStopsChange={setStops}
          />
        </section>
      </main>
    </div>
  )
}
