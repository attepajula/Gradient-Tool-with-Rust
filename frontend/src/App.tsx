import { useState, useEffect, useCallback, useRef } from 'react'
import { extractColors, renderGradient } from './api'
import ImageUpload from './components/ImageUpload'
import ColorPalette from './components/ColorPalette'
import GradientControls from './components/GradientControls'
import GradientPreview from './components/GradientPreview'

export type Paradigm = 'linear' | 'diagonal' | 'radial' | 'reflected'

export interface Stop {
  id: string
  hex: string
  position: number
}

let nextId = 1
export function makeStop(hex: string, position: number): Stop {
  return { id: String(nextId++), hex, position }
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
  const [gradientUrl, setGradientUrl] = useState<string | null>(null)
  const [rendering, setRendering] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const prevUrlRef = useRef<string | null>(null)

  const render = useCallback(async () => {
    if (stops.length === 0) return
    setRendering(true)
    setError(null)
    try {
      const blob = await renderGradient({
        stops: stops.map(s => ({ hex: s.hex, position: s.position })),
        width,
        height,
        paradigm,
        warp: 'none',
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
  }, [stops, paradigm, width, height])

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
      setStops(dominant_colors.map((hex, i) =>
        makeStop(hex, n === 1 ? 0 : i / (n - 1))
      ))
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
            onParadigmChange={setParadigm}
            onWidthChange={setWidth}
            onHeightChange={setHeight}
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
