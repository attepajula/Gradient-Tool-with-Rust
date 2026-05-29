import { Stop, makeStop } from '../App'

interface RowProps {
  stop: Stop
  onChange: (hex: string) => void
  onRemove: () => void
  canRemove: boolean
}

function ColorRow({ stop, onChange, onRemove, canRemove }: RowProps) {
  const id = `cp-${stop.id}`
  return (
    <div className="color-row">
      <label htmlFor={id} className="swatch" style={{ background: stop.hex }} title={stop.hex} />
      <span className="color-hex">{stop.hex}</span>
      <input
        id={id}
        type="color"
        value={stop.hex}
        onChange={(e) => onChange(e.target.value)}
        className="color-picker"
      />
      <button
        className="remove-btn"
        onClick={onRemove}
        title="Remove"
        disabled={!canRemove}
      >
        ×
      </button>
    </div>
  )
}

interface Props {
  stops: Stop[]
  onChange: (stops: Stop[]) => void
}

export default function ColorPalette({ stops, onChange }: Props) {
  const update = (id: string, hex: string) =>
    onChange(stops.map(s => s.id === id ? { ...s, hex } : s))

  const remove = (id: string) => onChange(stops.filter(s => s.id !== id))

  const add = () => {
    const pos = stops.length === 0 ? 1 : Math.min(1, stops[stops.length - 1].position + 0.1)
    onChange([...stops, makeStop('#ffffff', pos)])
  }

  return (
    <div className="card">
      <h2 className="card-title">Colors</h2>
      <div className="color-list">
        {stops.map(stop => (
          <ColorRow
            key={stop.id}
            stop={stop}
            onChange={(hex) => update(stop.id, hex)}
            onRemove={() => remove(stop.id)}
            canRemove={stops.length > 1}
          />
        ))}
      </div>
      <button className="add-btn" onClick={add}>+ Add color</button>
    </div>
  )
}
