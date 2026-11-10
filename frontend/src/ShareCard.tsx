import { useState, useEffect } from 'react'

type ShareCardPayload = {
  fromTopic: string
  toTopic: string
  steps: number
}

export default function ShareCard({ payload }: { payload: ShareCardPayload }) {
  const [shareText, setShareText] = useState('')

  useEffect(() => {
    const generateText = () => {
      const stepsText = payload.steps <= 1 ? '1 click' : `${payload.steps} clicks`
      setShareText(`I went from ${payload.fromTopic} to ${payload.toTopic} in ${stepsText} using RabitHole!`)
    }

    generateText()
  }, [payload])

  const handleShare = async () => {
    if (navigator.share) {
      try {
        await navigator.share({
          title: 'Check out my RabitHole map!',
          text: shareText,
          url: window.location.href,
        })
      } catch (err) {
        // User refused the share prompt
        console.log('Share cancelled')
      }
    } else {
      // Fallback: copy to clipboard
      await navigator.clipboard.writeText(shareText)
      alert('Link copied to clipboard!')
    }
  }

  return (
    <div style={{ padding: '24px', border: '1px solid #e5e4e7', borderRadius: '8px', background: '#f8f5f0', marginTop: '16px' }}>
      <h3>Share Your Map</h3>
      <p>{shareText || 'Generating share card...'}</p>
      <button
        onClick={handleShare}
        style={{
          width: '100%',
          marginTop: '12px',
          padding: '8px',
          background: '#aa3bff',
          color: 'white',
          border: 'none',
          borderRadius: '4px',
          cursor: 'pointer',
        }}
      >
        Share
      </button>
      {navigator.share ? (
        <p style={{ marginTop: '8px', fontSize: '12px', color: '#6b6375' }}>
          Tap the share button to share to WhatsApp/Instagram/X
        </p>
      ) : (
        <p style={{ marginTop: '8px', fontSize: '12px', color: '#6b6375' }}>
          Copied to clipboard (no Web Share API available)
        </p>
      )}
    </div>
  )
}