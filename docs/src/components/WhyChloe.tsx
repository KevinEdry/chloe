import Link from 'next/link'

export function WhyChloe() {
  return (
    <section className="relative max-w-[1208px] mx-auto px-6 py-20">
      {/* Header */}
      <div className="max-w-3xl mb-12">
        <h2 className="text-3xl md:text-4xl font-semibold text-[var(--color-text-primary)] mb-4">
          Why Chloe?
        </h2>
        <p className="text-lg text-[var(--color-text-secondary)] leading-relaxed">
          AI coding assistants are powerful, but they're limited to single sessions. We needed
          something fast, simple, and purpose-built for AI coding workflows.
        </p>
      </div>

      {/* Bento Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {/* Large Card - Free & Open Source */}
        <div className="md:col-span-2 lg:col-span-2 row-span-2 bg-gradient-to-br from-[var(--color-primary)]/10 to-[var(--color-primary)]/5 border border-[var(--color-primary)]/20 rounded-2xl p-8 flex flex-col">
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 rounded-lg bg-[var(--color-primary)]/10">
              <svg
                className="w-6 h-6 text-[var(--color-primary-light)]"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4"
                />
              </svg>
            </div>
            <h3 className="text-xl font-semibold text-[var(--color-text-primary)]">
              Free and Open Source
            </h3>
          </div>
          <p className="text-[var(--color-text-secondary)] leading-relaxed mb-6 flex-grow">
            Chloe is free because we believe developer tools should be accessible to everyone. No
            subscriptions, no credit cards, no vendor lock-in. Open source means you can inspect the
            code, contribute improvements, and build the features you need.
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-4 border-t border-[var(--color-border)]">
            <div>
              <div className="text-2xl font-bold text-[var(--color-primary-light)]">0</div>
              <div className="text-sm text-[var(--color-text-tertiary)]">Tracking or telemetry</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-[var(--color-primary-light)]">100%</div>
              <div className="text-sm text-[var(--color-text-tertiary)]">Local data storage</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-[var(--color-primary-light)]">MIT</div>
              <div className="text-sm text-[var(--color-text-tertiary)]">Licensed</div>
            </div>
          </div>
        </div>

        {/* Small Card - Simplicity */}
        <div className="bg-[var(--color-surface)]/50 border border-[var(--color-border)] rounded-2xl p-6 hover:border-[var(--color-primary)]/30 transition-colors">
          <div className="p-2 rounded-lg bg-[var(--color-surface)] w-fit mb-4">
            <svg
              className="w-5 h-5 text-[var(--color-primary-light)]"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z"
              />
            </svg>
          </div>
          <h3 className="text-lg font-semibold text-[var(--color-text-primary)] mb-2">
            Simplicity Over Complexity
          </h3>
          <p className="text-sm text-[var(--color-text-secondary)] leading-relaxed">
            Sensible defaults, vim-style navigation, zero configuration required. No prefix keys to
            memorize.
          </p>
        </div>

        {/* Small Card - Performance */}
        <div className="bg-[var(--color-surface)]/50 border border-[var(--color-border)] rounded-2xl p-6 hover:border-[var(--color-primary)]/30 transition-colors">
          <div className="p-2 rounded-lg bg-[var(--color-surface)] w-fit mb-4">
            <svg
              className="w-5 h-5 text-[var(--color-primary-light)]"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
          </div>
          <h3 className="text-lg font-semibold text-[var(--color-text-primary)] mb-2">
            Performance Over Features
          </h3>
          <p className="text-sm text-[var(--color-text-secondary)] leading-relaxed">
            Native Rust binary, ~5MB memory footprint, instant startup. No Electron bloat.
          </p>
        </div>

        {/* Medium Card - Safety */}
        <div className="lg:col-span-2 bg-[var(--color-surface)]/50 border border-[var(--color-border)] rounded-2xl p-6 hover:border-[var(--color-primary)]/30 transition-colors">
          <div className="flex flex-col sm:flex-row sm:items-start gap-4">
            <div className="p-2 rounded-lg bg-[var(--color-surface)] w-fit flex-shrink-0">
              <svg
                className="w-5 h-5 text-[var(--color-primary-light)]"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-[var(--color-text-primary)] mb-2">
                Safety First
              </h3>
              <p className="text-sm text-[var(--color-text-secondary)] leading-relaxed">
                100% safe Rust code, no unsafe blocks, memory-safe by design. No buffer overflows,
                no data races, compiler-enforced safety guarantees.
              </p>
            </div>
          </div>
        </div>

        {/* Small Card - Community */}
        <div className="bg-[var(--color-surface)]/50 border border-[var(--color-border)] rounded-2xl p-6 hover:border-[var(--color-primary)]/30 transition-colors">
          <div className="p-2 rounded-lg bg-[var(--color-surface)] w-fit mb-4">
            <svg
              className="w-5 h-5 text-[var(--color-primary-light)]"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
              />
            </svg>
          </div>
          <h3 className="text-lg font-semibold text-[var(--color-text-primary)] mb-2">
            Community Over Profit
          </h3>
          <p className="text-sm text-[var(--color-text-secondary)] leading-relaxed">
            Built with community contributions, no investors, no exit strategy.
          </p>
        </div>
      </div>

      {/* CTAs */}
      <div className="flex flex-col sm:flex-row gap-4 justify-center pt-12">
        <Link
          href="https://github.com/kevinedry/chloe"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center justify-center gap-2 px-6 py-3 rounded-lg bg-[var(--color-primary)]/10 border border-[var(--color-primary)]/20 text-[var(--color-primary-light)] hover:bg-[var(--color-primary)]/15 hover:border-[var(--color-primary)]/30 transition-all font-medium"
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
            <path
              fillRule="evenodd"
              clipRule="evenodd"
              d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.17 6.839 9.49.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.167 22 16.418 22 12c0-5.523-4.477-10-10-10z"
            />
          </svg>
          View on GitHub
        </Link>
        <Link
          href="/docs/quick-start"
          className="inline-flex items-center justify-center gap-2 px-6 py-3 rounded-lg bg-[var(--color-primary)] text-white hover:bg-[var(--color-primary)]/90 transition-all font-medium"
        >
          Get Started
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </Link>
      </div>
    </section>
  )
}
