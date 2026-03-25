interface Props {
  url: string | null
  loading: boolean
  error: string | null
}

export default function GradientPreview({ url, loading, error }: Props) {
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
          <div className="preview-error">{error}</div>
        ) : url ? (
          <img
            src={url}
            alt="Gradient preview"
            className={`preview-img ${loading ? 'preview-updating' : ''}`}
          />
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
