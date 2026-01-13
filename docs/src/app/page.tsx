import type { Metadata } from 'next'
import { Demo } from '@/components/Demo'
import { Features } from '@/components/Features'
import { Faq } from '@/components/faq/Faq'
import { Hero } from '@/components/hero/Hero'

export const metadata: Metadata = {
  alternates: {
    canonical: '/',
  },
}

const jsonLd = {
  '@context': 'https://schema.org',
  '@type': 'SoftwareApplication',
  name: 'Chloe',
  applicationCategory: 'DeveloperApplication',
  operatingSystem: 'macOS, Linux, Windows',
  description:
    'A terminal multiplexer for AI coding agents. Run Claude Code, Gemini CLI, Amp, OpenCode, and more in parallel with integrated Kanban task management.',
  url: 'https://getchloe.sh',
  author: {
    '@type': 'Person',
    name: 'Kevin Edry',
    url: 'https://kevin-edry.com',
  },
  offers: {
    '@type': 'Offer',
    price: '0',
    priceCurrency: 'USD',
  },
  programmingLanguage: 'Rust',
}

export default function LandingPage() {
  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <div className="min-h-screen bg-[var(--color-background)]">
        <Hero />
        <Demo />
        <Features />
        <Faq />
      </div>
    </>
  )
}
