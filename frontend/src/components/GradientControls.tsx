import { Paradigm, Warp } from '../App'

interface Props {
  paradigm: Paradigm
  warp: Warp
  width: number
  height: number
  onParadigmChange: (v: Paradigm) => void
  onWarpChange: (v: Warp) => void
  onWidthChange: (v: number) => void
  onHeightChange: (v: number) => void
}

const PARADIGMS: { value: Paradigm; label: string; description: string }[] = [
  { value: 'linear',   label: 'Linear',   description: 'Left → right' },
  { value: 'diagonal', label: 'Diagonal', description: 'Top-left → bottom-right' },
  { value: 'radial',   label: 'Radial',   description: 'Center → edges' },
  { value: 'reflected',label: 'Reflected',description: 'Edges → center' },
]

const WARPS: { value: Warp; label: string; description: string }[] = [
  { value: 'none',        label: 'None',        description: 'Straight interpolation' },
  { value: 'ease_in',     label: 'Ease In',     description: 'Slow start' },
  { value: 'ease_out',    label: 'Ease Out',    description: 'Slow end' },
  { value: 'ease_in_out', label: 'Ease In/Out', description: 'Slow at both ends' },
  { value: 'wave',        label: 'Wave',        description: 'Sinusoidal oscillation' },
  { value: 'zigzag',      label: 'Zigzag',      description: 'Bounce back and forth' },
]

export default function GradientControls({
  paradigm, warp, width, height,
  onParadigmChange, onWarpChange, onWidthChange, onHeightChange,
}: Props) {
  return (
    <div className="card controls-card">
      <div className="controls-grid">
        <div className="control-group">
          <label>Paradigm</label>
          <select value={paradigm} onChange={(e) => onParadigmChange(e.target.value as Paradigm)}>
            {PARADIGMS.map(({ value, label, description }) => (
              <option key={value} value={value}>{label} — {description}</option>
            ))}
          </select>
        </div>

        <div className="control-group">
          <label>Warp</label>
          <select value={warp} onChange={(e) => onWarpChange(e.target.value as Warp)}>
            {WARPS.map(({ value, label, description }) => (
              <option key={value} value={value}>{label} — {description}</option>
            ))}
          </select>
        </div>

        <div className="control-group">
          <label>Width (px)</label>
          <input
            type="number"
            min={1} max={4096}
            value={width}
            onChange={(e) => onWidthChange(Number(e.target.value))}
          />
        </div>

        <div className="control-group">
          <label>Height (px)</label>
          <input
            type="number"
            min={1} max={4096}
            value={height}
            onChange={(e) => onHeightChange(Number(e.target.value))}
          />
        </div>
      </div>
    </div>
  )
}
