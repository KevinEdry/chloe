import { Metadata } from 'next'
import { Hero } from '@/components/hero/Hero'
import { Demo } from '@/components/Demo'
import { Features } from '@/components/Features'
import { Faq } from '@/components/faq/Faq'

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
    'A terminal multiplexer for running parallel Claude Code instances with integrated Kanban task management.',
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
