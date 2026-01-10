import { Demo, Features, Hero } from '@/components'

export default function LandingPage() {
  return (
    <div className="min-h-screen bg-[var(--color-background)]">
      <Hero />
      <Demo />
      <Features />
    </div>
  )
}
