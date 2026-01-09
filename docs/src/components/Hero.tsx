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
          <div className="mt-8 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
            <Link
              href="/docs"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-lg text-base font-medium bg-gradient-to-b from-[var(--color-primary)] to-[var(--color-primary-dark)] text-white shadow-lg shadow-[var(--color-primary)]/25 hover:shadow-[var(--color-primary)]/40 hover:-translate-y-0.5 transition-all"
            >
              Get started
              <span aria-hidden="true">→</span>
            </Link>
            <a
              href="https://github.com/KevinEdry/chloe"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-lg text-base font-medium bg-white/5 border border-white/10 text-[var(--color-text-secondary)] hover:bg-white/10 hover:text-[var(--color-text-primary)] transition-all"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
              </svg>
              View on GitHub
            </a>
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
