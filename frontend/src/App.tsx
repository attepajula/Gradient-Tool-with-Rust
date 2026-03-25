import { useState, useEffect, useCallback, useRef } from 'react'
import { extractColors, renderGradient } from './api'
import ImageUpload from './components/ImageUpload'
import ColorPalette from './components/ColorPalette'
import GradientControls from './components/GradientControls'
import GradientPreview from './components/GradientPreview'

export type Paradigm = 'linear' | 'diagonal' | 'radial' | 'reflected'
export type Warp = 'none' | 'ease_in' | 'ease_out' | 'ease_in_out' | 'wave' | 'zigzag'

export default function App() {
  const [colors, setColors] = useState<string[]>(['#e8534a', '#f0a500', '#4a90d9'])
  const [paradigm, setParadigm] = useState<Paradigm>('linear')
  const [warp, setWarp] = useState<Warp>('none')
  const [width, setWidth] = useState(1200)
  const [height, setHeight] = useState(200)
  const [gradientUrl, setGradientUrl] = useState<string | null>(null)
  const [rendering, setRendering] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const prevUrlRef = useRef<string | null>(null)

  const render = useCallback(async () => {
    if (colors.length === 0) return
    setRendering(true)
    setError(null)
    try {
      const blob = await renderGradient({ colors, width, height, paradigm, warp, quality: 92 })
      const url = URL.createObjectURL(blob)
      if (prevUrlRef.current) URL.revokeObjectURL(prevUrlRef.current)
      prevUrlRef.current = url
      setGradientUrl(url)
    } catch (e) {
      setError(String(e))
    } finally {
      setRendering(false)
    }
  }, [colors, paradigm, warp, width, height])

  // Debounced auto-render on any change
  useEffect(() => {
    const t = setTimeout(render, 250)
    return () => clearTimeout(t)
  }, [render])

  const handleImageUpload = async (file: File) => {
    setUploading(true)
    setError(null)
    try {
      const { dominant_colors } = await extractColors(file)
      setColors(dominant_colors)
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
          <ColorPalette colors={colors} onChange={setColors} />
        </aside>

        <section className="content">
          <GradientControls
            paradigm={paradigm}
            warp={warp}
            width={width}
            height={height}
            onParadigmChange={setParadigm}
            onWarpChange={setWarp}
            onWidthChange={setWidth}
            onHeightChange={setHeight}
          />
          <GradientPreview
            url={gradientUrl}
            loading={rendering}
            error={error}
          />
        </section>
      </main>
    </div>
  )
}
