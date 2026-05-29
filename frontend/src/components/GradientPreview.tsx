import { useRef, useCallback } from 'react'
import { Stop, Paradigm } from '../App'

interface Props {
  url: string | null
  loading: boolean
  error: string | null
  stops: Stop[]
  paradigm: Paradigm
  onStopsChange: (stops: Stop[]) => void
}

function stopToXY(stop: Stop, paradigm: Paradigm): { x: number; y: number } {
  const t = stop.position
  switch (paradigm) {
    case 'linear':
      return { x: t, y: 0.5 }
    case 'diagonal':
      return { x: t, y: t }
    case 'radial':
      return { x: 0.5 + t * 0.5, y: 0.5 }
    case 'reflected':
      return { x: 0.5 + (1 - t) * 0.5, y: 0.5 }
  }
}

function xyToT(x: number, y: number, paradigm: Paradigm): number {
  const clamp = (v: number) => Math.max(0, Math.min(1, v))
  switch (paradigm) {
    case 'linear':
      return clamp(x)
    case 'diagonal':
      return clamp((x + y) / 2)
    case 'radial':
      return clamp(2 * Math.sqrt((Math.max(0.5, x) - 0.5) ** 2 + (y - 0.5) ** 2))
    case 'reflected':
      return clamp(1 - 2 * Math.sqrt((Math.max(0.5, x) - 0.5) ** 2 + (y - 0.5) ** 2))
  }
}

export default function GradientPreview({ url, loading, error, stops, paradigm, onStopsChange }: Props) {
  const wrapRef = useRef<HTMLDivElement>(null)
  const draggingId = useRef<string | null>(null)
  const stopsRef = useRef(stops)
  stopsRef.current = stops

  const onMouseMove = useCallback((e: MouseEvent) => {
    const id = draggingId.current
    if (!id || !wrapRef.current) return
    const rect = wrapRef.current.getBoundingClientRect()
    const x = (e.clientX - rect.left) / rect.width
    const y = (e.clientY - rect.top) / rect.height
    const t = xyToT(x, y, paradigm)
    onStopsChange(stopsRef.current.map(s => s.id === id ? { ...s, position: t } : s))
  }, [paradigm, onStopsChange])

  const onMouseUp = useCallback(() => {
    draggingId.current = null
    window.removeEventListener('mousemove', onMouseMove)
    window.removeEventListener('mouseup', onMouseUp)
  }, [onMouseMove])

  const startDrag = (id: string) => (e: React.MouseEvent) => {
    e.preventDefault()
    draggingId.current = id
    window.addEventListener('mousemove', onMouseMove)
    window.addEventListener('mouseup', onMouseUp)
  }

  const download = () => {
    if (!url) return
    const a = document.createElement('a')
    a.href = url
    a.download = 'gradient.jpg'
    a.click()
  }

  return (
    <div className="card preview-card">
      <div className="preview-area">
        {error ? (
          <div className="preview-error">
            <span className="error-shrug">¯\_(ツ)_/¯</span>
            <span className="error-msg">something went wrong</span>
          </div>
        ) : url ? (
          <div ref={wrapRef} className="preview-img-wrap">
            <img
              src={url}
              alt="Gradient preview"
              className={`preview-img ${loading ? 'preview-updating' : ''}`}
              draggable={false}
            />
            {stops.map(stop => {
              const { x, y } = stopToXY(stop, paradigm)
              return (
                <div
                  key={stop.id}
                  className="gradient-point"
                  style={{ left: `${x * 100}%`, top: `${y * 100}%`, background: stop.hex }}
                  onMouseDown={startDrag(stop.id)}
                />
              )
            })}
          </div>
        ) : (
          <div className="preview-placeholder">
            {loading ? <div className="spinner" /> : <span>Gradient will appear here</span>}
          </div>
        )}
      </div>

      <div className="preview-footer">
        <button className="download-btn" onClick={download} disabled={!url || loading}>
          ↓ Download gradient.jpg
        </button>
      </div>
    </div>
  )
}
