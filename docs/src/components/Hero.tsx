'use client'

import Link from 'next/link'

export function Hero() {
  return (
    <section className="relative overflow-hidden">
      {/* Gradient overlay - fades tiles from top */}
      <div
        className="absolute inset-0 z-[1]"
        style={{
          backgroundImage: 'linear-gradient(180deg, rgba(10, 10, 15, 1) 0%, rgba(10, 10, 15, 0) 100%)',
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
            <span className="hidden sm:inline"><br /></span>
            parallel Claude Code instances with integrated task management.
          </p>

          {/* CTA buttons */}
          <div className="mt-8 flex flex-col items-center gap-4 sm:flex-row sm:justify-center sm:items-center">
            <button
              onClick={() => navigator.clipboard.writeText('curl -fsSL getchloe.sh/install.sh | bash')}
              className="inline-flex items-center gap-2 px-6 py-3 rounded-lg text-base font-medium bg-gradient-to-b from-[var(--color-primary)] to-[var(--color-primary-dark)] text-white shadow-lg shadow-[var(--color-primary)]/25 hover:shadow-[var(--color-primary)]/40 hover:-translate-y-0.5 transition-all"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              Install
            </button>
            <span className="text-[var(--color-text-muted)]">or</span>
            <Link
              href="/getting-started"
              className="inline-flex items-center gap-2 text-base font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors"
            >
              Get started
              <span aria-hidden="true">→</span>
            </Link>
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
