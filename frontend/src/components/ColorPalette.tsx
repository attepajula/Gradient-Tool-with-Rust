interface RowProps {
  id: string
  color: string
  onChange: (hex: string) => void
  onRemove: () => void
  canRemove: boolean
}

function ColorRow({ id, color, onChange, onRemove, canRemove }: RowProps) {
  return (
    <div className="color-row">
      <label htmlFor={id} className="swatch" style={{ background: color }} title={color} />
      <span className="color-hex">{color}</span>
      <input
        id={id}
        type="color"
        value={color}
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
  colors: string[]
  onChange: (colors: string[]) => void
}

export default function ColorPalette({ colors, onChange }: Props) {
  const update = (i: number, hex: string) => {
    const next = [...colors]
    next[i] = hex
    onChange(next)
  }

  const remove = (i: number) => onChange(colors.filter((_, idx) => idx !== i))

  const add = () => onChange([...colors, '#ffffff'])

  return (
    <div className="card">
      <h2 className="card-title">Colors</h2>
      <div className="color-list">
        {colors.map((color, i) => (
          <ColorRow
            key={i}
            id={`color-picker-${i}`}
            color={color}
            onChange={(hex) => update(i, hex)}
            onRemove={() => remove(i)}
            canRemove={colors.length > 1}
          />
        ))}
      </div>
      <button className="add-btn" onClick={add}>+ Add color</button>
    </div>
  )
}
