import { useRef, useState, DragEvent, ChangeEvent } from 'react'

interface Props {
  onUpload: (file: File) => void
  loading: boolean
}

export default function ImageUpload({ onUpload, loading }: Props) {
  const inputRef = useRef<HTMLInputElement>(null)
  const [dragging, setDragging] = useState(false)
  const [preview, setPreview] = useState<string | null>(null)

  const handle = (file: File) => {
    if (!file.type.startsWith('image/')) return
    const url = URL.createObjectURL(file)
    setPreview(url)
    onUpload(file)
  }

  const onDrop = (e: DragEvent) => {
    e.preventDefault()
    setDragging(false)
    const file = e.dataTransfer.files[0]
    if (file) handle(file)
  }

  const onChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) handle(file)
  }

  return (
    <div className="card">
      <h2 className="card-title">Photo</h2>
      <div
        className={`drop-zone ${dragging ? 'dragging' : ''} ${loading ? 'loading' : ''}`}
        onClick={() => inputRef.current?.click()}
        onDragOver={(e) => { e.preventDefault(); setDragging(true) }}
        onDragLeave={() => setDragging(false)}
        onDrop={onDrop}
      >
        {loading ? (
          <div className="drop-zone-content">
            <div className="spinner" />
            <span>Extracting colors…</span>
          </div>
        ) : preview ? (
          <img src={preview} alt="Uploaded" className="drop-zone-preview" />
        ) : (
          <div className="drop-zone-content">
            <span className="drop-icon">↑</span>
            <span>Drop image or click to upload</span>
            <span className="drop-hint">JPEG · PNG</span>
          </div>
        )}
      </div>
      <input
        ref={inputRef}
        type="file"
        accept="image/jpeg,image/png"
        style={{ display: 'none' }}
        onChange={onChange}
      />
    </div>
  )
}
