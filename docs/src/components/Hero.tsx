'use client'

import clsx from 'clsx'
import Link from 'next/link'
import { useState } from 'react'

type Platform = 'macos' | 'linux' | 'windows'

const platforms: Record<Platform, { label: string; command: string }> = {
  macos: {
    label: 'macOS',
    command: 'curl -fsSL getchloe.sh/install.sh | bash',
  },
  linux: {
    label: 'Linux',
    command: 'curl -fsSL getchloe.sh/install.sh | bash',
  },
  windows: {
    label: 'Windows',
    command: 'irm getchloe.sh/install.ps1 | iex',
  },
}

export function Hero() {
  const [copied, setCopied] = useState(false)
  const [selectedPlatform, setSelectedPlatform] = useState<Platform>('macos')

  const handleCopy = async () => {
    const command = platforms[selectedPlatform].command
    try {
      await navigator.clipboard.writeText(command)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch {
      const textArea = document.createElement('textarea')
      textArea.value = command
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    }
  }

  return (
    <section className="relative overflow-hidden">
      {/* Gradient overlay - fades tiles from top */}
      <div
        className="absolute inset-0 z-[1]"
        style={{
          backgroundImage:
            'linear-gradient(180deg, rgba(10, 10, 15, 1) 0%, rgba(10, 10, 15, 0) 100%)',
        }}
      />

      {/* Tiled background pattern - hard edge at bottom */}
      <div
        className="absolute inset-0 z-0"
        style={{
          backgroundImage: `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='70' height='70'%3E%3Cpath d='M70 0 V70 H0' fill='none' stroke='%23a855f7' stroke-opacity='0.3'/%3E%3C/svg%3E")`,
          backgroundSize: '70px 70px',
          backgroundPosition: 'center bottom',
          backgroundRepeat: 'repeat',
        }}
      />

      {/* Content */}
      <div className="relative z-[2] px-6 pt-16 pb-36 md:pt-24 md:pb-44">
        <div className="mx-auto max-w-3xl text-center">
          {/* Announcement badge */}
          <div className="mb-8">
            <Link
              href="/docs"
              className="inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm bg-[var(--color-primary)]/10 border border-[var(--color-primary)]/20 text-[var(--color-primary-light)] hover:bg-[var(--color-primary)]/15 hover:border-[var(--color-primary)]/30 transition-all"
            >
              <span>100% Safe Rust</span>
              <span className="opacity-60">·</span>
              <span>Vim-style navigation</span>
              <span className="ml-1">→</span>
            </Link>
          </div>

          {/* Headline */}
          <h1 className="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl leading-[1.1]">
            <span className="bg-gradient-to-b from-[var(--color-text-primary)] to-[#a1a1aa] bg-clip-text text-transparent">
              A better way to multiplex
            </span>
            <br />
            <span className="bg-gradient-to-b from-[var(--color-text-primary)] to-[#a1a1aa] bg-clip-text text-transparent">
              Claude Code instances
            </span>
          </h1>

          {/* Subtitle */}
          <p className="mt-6 text-lg text-[var(--color-text-secondary)] sm:text-xl max-w-2xl mx-auto leading-relaxed">
            Chloe is a terminal multiplexer for running{' '}
            <span className="hidden sm:inline">
              <br />
            </span>
            parallel Claude Code instances with integrated task management.
          </p>

          {/* Installation command - primary CTA */}
          <div className="mt-10 flex flex-col items-center gap-4">
            <div className="inline-flex items-stretch rounded-xl overflow-hidden border border-white/10">
              {/* Platform dropdown */}
              <div className="relative group/dropdown">
                <div className="h-full flex items-center gap-2 px-4 py-3 text-sm font-medium bg-white text-black hover:bg-[#e5e5e5] transition-colors cursor-pointer">
                  <span>Get Chloe</span>
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </div>

                {/* Dropdown menu */}
                <div className="hidden group-hover/dropdown:block absolute top-full left-0 pt-2 w-36 z-50">
                  <div className="rounded-lg bg-[#18181b] border border-white/10 shadow-2xl overflow-hidden">
                    {(Object.keys(platforms) as Platform[]).map((platform) => (
                      <button
                        key={platform}
                        type="button"
                        onClick={() => setSelectedPlatform(platform)}
                        className={clsx(
                          'w-full px-4 py-2.5 text-left text-sm transition-colors cursor-pointer',
                          selectedPlatform === platform
                            ? 'bg-[var(--color-primary)]/20 text-[var(--color-primary-light)]'
                            : 'text-[var(--color-text-secondary)] hover:bg-white/5 hover:text-[var(--color-text-primary)]',
                        )}
                      >
                        {platforms[platform].label}
                      </button>
                    ))}
                  </div>
                </div>
              </div>

              {/* Command card - clickable to copy */}
              <button
                type="button"
                onClick={handleCopy}
                className="group flex items-center gap-4 px-5 py-3 bg-white/[0.03] hover:bg-white/[0.06] transition-colors cursor-pointer"
              >
                {/* Command with syntax highlighting */}
                <code className="font-mono text-base">
                  {selectedPlatform === 'windows' ? (
                    <>
                      <span className="text-[var(--color-primary-light)]">irm</span>
                      <span className="text-[var(--color-text-muted)]"> </span>
                      <span className="text-white">getchloe.sh/install.ps1</span>
                      <span className="text-sky-400"> | iex</span>
                    </>
                  ) : (
                    <>
                      <span className="text-[var(--color-primary-light)]">curl -fsSL</span>
                      <span className="text-[var(--color-text-muted)]"> </span>
                      <span className="text-white">getchloe.sh/install.sh</span>
                      <span className="text-sky-400"> | bash</span>
                    </>
                  )}
                </code>

                {/* Copy icon */}
                <div className="flex items-center pl-2">
                  {copied ? (
                    <svg
                      className="w-5 h-5 text-[var(--color-primary)]"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M5 13l4 4L19 7"
                      />
                    </svg>
                  ) : (
                    <svg
                      className="w-5 h-5 text-[var(--color-text-muted)] group-hover:text-[var(--color-text-secondary)] hover:!text-[var(--color-primary-light)] transition-colors"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                      />
                    </svg>
                  )}
                </div>
              </button>
            </div>

            {/* Secondary link */}
            <div className="flex items-center gap-1 text-sm text-[var(--color-text-muted)]">
              <span>Or read the</span>
              <Link
                href="/getting-started"
                className="text-[var(--color-primary-light)] hover:text-[var(--color-primary)] underline underline-offset-2 decoration-[var(--color-primary-light)]/30 hover:decoration-[var(--color-primary)]/50 transition-colors"
              >
                documentation
              </Link>
            </div>
          </div>

          {/* Stats/links row */}
          <div className="mt-8 flex items-center justify-center gap-6 text-sm text-[var(--color-text-muted)]">
            <a
              href="https://github.com/KevinEdry/chloe"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-1.5 hover:text-[var(--color-text-secondary)] transition-colors"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z" />
              </svg>
              Star on GitHub
            </a>
            <span className="opacity-40">·</span>
            <span>Built with Rust</span>
          </div>
        </div>
      </div>
    </section>
  )
}
