import { useState } from 'react'
import axios from 'axios'

type WaitlistPayload = {
  email: string
}

export default function LandingPage() {
  const [email, setEmail] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [success, setSuccess] = useState(false)
  const [error, setError] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!email.trim()) return

    setSubmitting(true)
    setError(false)
    try {
      const response = await axios.post('/api/waitlist', { email: email.trim() })
      setSuccess(true)
      setEmail('')
    } catch (err) {
      setError(true)
      console.error('Waitlist error:', err)
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div style={{ minHeight: '100vh', display: 'flex', flexDirection: 'column', justifyContent: 'center', alignItems: 'center', background: '#f8f5f0', padding: '20px' }}>
      <div style={{ maxWidth: '600px', textAlign: 'center', marginBottom: '40px' }}>
        <h1 style={{ fontSize: '32px', fontWeight: 'bold', marginBottom: '16px', color: '#1a1a2e' }}>RabitHole</h1>
        <h2 style={{ fontSize: '20px', fontWeight: '500', marginBottom: '32px', color: '#6b7280' }}>Turn wandering into learning</h2>
        <p style={{ fontSize: '16px', marginBottom: '32px', color: '#6b7280' }}>
          Explore topics, build your curiosity graph, and track your learning streak. Free to start, paid for power users.
        </p>
      </div>

      {success ? (
        <div style={{ marginBottom: '32px', padding: '16px', background: '#ecfdf5', borderRadius: '8px', border: '1px solid #a7f3d0' }}>
          <p style={{ margin: '0', color: '#059669' }}>Thanks! You've been added to the waitlist. We'll email you when we launch.</p>
        </div>
      ) : (
        <form onSubmit={handleSubmit} style={{ maxWidth: '500px', width: '100%', display: 'flex', gap: '12px', marginBottom: '32px' }}>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="Enter your email"
            style={{ flex: 1, padding: '12px', border: '1px solid #e5e4e7', borderRadius: '6px', fontSize: '16px' }}
            required
          />
          <button
            type="submit"
            style={{
              padding: '12px 24px',
              background: '#8b5cf6',
              color: 'white',
              border: 'none',
              borderRadius: '6px',
              fontSize: '16px',
              fontWeight: '500',
              cursor: 'pointer',
            }}
            disabled={submitting}
          >
            {submitting ? 'Submitting...' : 'Get Early Access'}
          </button>
        </form>
      )}

      {error && (
        <div style={{ marginBottom: '32px', padding: '16px', background: '#fef2f2', borderRadius: '8px', border: '1px solid #fca5a5' }}>
          <p style={{ margin: '0', color: '#dc2626' }}>Something went wrong. Please try again.</p>
        </div>
      )}

      <div style={{ marginTop: '40px', color: '#6b7280', fontSize: '14px' }}>
        <p>Already have an account? <a href="#/" style={{ color: '#8b5cf6', textDecoration: 'none' }}>Sign in</a></p>
        <p>Curious about the app? <a href="#/" style={{ color: '#8b5cf6', textDecoration: 'none' }}>Learn more</a></p>
      </div>
    </div>
  )
}