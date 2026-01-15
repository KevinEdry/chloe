import type { Metadata } from 'next'
import { Demo } from '@/components/Demo'
import { Features } from '@/components/Features'
import { WhyChloe } from '@/components/WhyChloe'
import { HowItWorks } from '@/components/HowItWorks'
import { InstallBanner } from '@/components/InstallBanner'
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
  name: 'Chloe - Terminal Multiplexer for AI Coding Agents',
  alternateName: 'Chloe',
  applicationCategory: 'DeveloperApplication',
  operatingSystem: 'macOS, Linux, Windows',
  description:
    'Terminal multiplexer for AI coding agents. Run Claude Code, Gemini CLI, Amp, OpenCode, and more in parallel with integrated Kanban task management. Built for software engineers. Free and open source. Supports Git worktrees and Jujutsu workspaces.',
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
  keywords: [
    'terminal multiplexer',
    'AI coding agents',
    'Claude Code',
    'OpenCode',
    'Gemini CLI',
    'Amp',
    'task management',
    'kanban board',
    'git worktrees',
    'developer tools',
  ],
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
        <WhyChloe />
        <HowItWorks />
        <Features />
        {/* CTA Banner */}
        <InstallBanner />
        <Faq />
      </div>
    </>
  )
}
