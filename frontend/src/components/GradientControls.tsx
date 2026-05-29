import { useState, useEffect, useRef } from 'react'
import { Paradigm, Stop, makeStop } from '../App'

interface Props {
  paradigm: Paradigm
  width: number
  height: number
  noise: number
  onParadigmChange: (v: Paradigm) => void
  onWidthChange: (v: number) => void
  onHeightChange: (v: number) => void
  onNoiseChange: (v: number) => void
  onStopsChange: (stops: Stop[]) => void
}

function SizeInput({
  label, value, min, max, onChange,
}: { label: string; value: number; min: number; max: number; onChange: (v: number) => void }) {
  const [raw, setRaw] = useState(String(value))
  const focusedRef = useRef(false)

  useEffect(() => {
    if (!focusedRef.current) setRaw(String(value))
  }, [value])

  const commit = (s: string) => {
    const n = parseInt(s, 10)
    if (!isNaN(n)) {
      const clamped = Math.min(Math.max(n, min), max)
      onChange(clamped)
      setRaw(String(clamped))
    } else {
      setRaw(String(value))
    }
  }

  return (
    <div className="control-group">
      <label>{label}</label>
      <input
        type="number"
        min={min} max={max}
        value={raw}
        onChange={(e) => setRaw(e.target.value)}
        onFocus={() => { focusedRef.current = true }}
        onBlur={(e) => { focusedRef.current = false; commit(e.target.value) }}
        onKeyDown={(e) => { if (e.key === 'Enter') commit((e.target as HTMLInputElement).value) }}
      />
    </div>
  )
}

const PARADIGMS: { value: Paradigm; label: string; description: string }[] = [
  { value: 'linear',    label: 'Linear',    description: 'Left → right' },
  { value: 'diagonal',  label: 'Diagonal',  description: 'Corner → corner' },
  { value: 'radial',    label: 'Radial',    description: 'Center → edges' },
  { value: 'reflected', label: 'Reflected', description: 'Edges → center' },
]

interface Preset { label: string; stops: { hex: string; position: number }[] }

const PRESETS: Preset[] = [
  { label: 'Sunset',     stops: [{ hex: '#1a0533', position: 0 }, { hex: '#c0392b', position: 0.3 }, { hex: '#e67e22', position: 0.6 }, { hex: '#f9ca24', position: 1 }] },
  { label: 'Ocean',      stops: [{ hex: '#0d1b2a', position: 0 }, { hex: '#1565c0', position: 0.4 }, { hex: '#29b6f6', position: 0.75 }, { hex: '#e0f7fa', position: 1 }] },
  { label: 'Forest',     stops: [{ hex: '#1b2a1b', position: 0 }, { hex: '#2e7d32', position: 0.45 }, { hex: '#a5d6a7', position: 1 }] },
  { label: 'Neon',       stops: [{ hex: '#ff00ff', position: 0 }, { hex: '#00ffff', position: 0.5 }, { hex: '#ff00ff', position: 1 }] },
  { label: 'Monochrome', stops: [{ hex: '#111111', position: 0 }, { hex: '#888888', position: 0.5 }, { hex: '#ffffff', position: 1 }] },
  { label: 'Candy',      stops: [{ hex: '#ff6b9d', position: 0 }, { hex: '#c44dff', position: 0.35 }, { hex: '#4d79ff', position: 0.7 }, { hex: '#00d4aa', position: 1 }] },
]

export default function GradientControls({
  paradigm, width, height, noise,
  onParadigmChange, onWidthChange, onHeightChange, onNoiseChange, onStopsChange,
}: Props) {
  const applyPreset = (label: string) => {
    const preset = PRESETS.find(p => p.label === label)
    if (preset) onStopsChange(preset.stops.map(s => makeStop(s.hex, s.position)))
  }

  return (
    <div className="card controls-card">
      <div className="controls-grid">
        <div className="control-group">
          <label>Style</label>
          <select value={paradigm} onChange={(e) => onParadigmChange(e.target.value as Paradigm)}>
            {PARADIGMS.map(({ value, label, description }) => (
              <option key={value} value={value}>{label} — {description}</option>
            ))}
          </select>
        </div>

        <div className="control-group">
          <label>Colors</label>
          <select value="" onChange={(e) => { if (e.target.value) applyPreset(e.target.value) }}>
            <option value="" disabled>Choose preset…</option>
            {PRESETS.map(({ label }) => (
              <option key={label} value={label}>{label}</option>
            ))}
          </select>
        </div>

        <SizeInput label="Width (px)"  value={width}  min={1} max={4096} onChange={onWidthChange} />
        <SizeInput label="Height (px)" value={height} min={1} max={4096} onChange={onHeightChange} />

        <div className="control-group noise-group">
          <label>Noise <span className="noise-value">{Math.round(noise * 100)}%</span></label>
          <input
            type="range"
            min={0} max={1} step={0.01}
            value={noise}
            onChange={(e) => onNoiseChange(parseFloat(e.target.value))}
          />
        </div>
      </div>
    </div>
  )
}
