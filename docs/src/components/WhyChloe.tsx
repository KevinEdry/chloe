import Link from 'next/link'

export function WhyChloe() {
  return (
    <section className="relative max-w-[1208px] mx-auto px-6 py-20">
      <h2 className="text-2xl font-semibold text-[var(--color-text-primary)] pb-8 text-center">
        Why Chloe?
      </h2>

      <div className="max-w-3xl mx-auto space-y-6">
        <div className="text-[var(--color-text-secondary)] leading-relaxed space-y-4">
          <p>
            AI coding assistants like Claude Code, OpenCode, and Gemini CLI are powerful, but
            they're limited to single sessions. Want to work on multiple features in parallel? You
            need to manage multiple terminals, track which task is where, and constantly switch
            context.
          </p>

          <p>
            Existing terminal multiplexers like tmux work, but they have steep learning curves with
            complex prefix keys and configuration files. Commercial alternatives add unnecessary
            bloat with Electron frameworks, consuming hundreds of megabytes of RAM just to display
            text.
          </p>

          <p className="text-[var(--color-text-primary)] font-medium">
            We needed something fast, simple, and purpose-built for AI coding workflows.
          </p>
        </div>

        <div className="bg-[var(--color-surface)]/50 border border-[var(--color-border)] rounded-lg p-8 space-y-4">
          <h3 className="text-xl font-semibold text-[var(--color-text-primary)]">
            Free and Open Source
          </h3>

          <div className="text-[var(--color-text-secondary)] leading-relaxed space-y-3">
            <p>
              Chloe is free because we believe developer tools should be accessible to everyone. No
              subscriptions, no credit cards, no vendor lock-in.
            </p>

            <p>
              Open source means you can inspect the code, contribute improvements, and build the
              features you need. No tracking, no telemetry, no data collection. Your work stays on
              your machine.
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 pt-4">
          <div className="space-y-2">
            <div className="flex items-center gap-2">
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
                  d="M5 13l4 4L19 7"
                />
              </svg>
              <h4 className="font-semibold text-[var(--color-text-primary)]">
                Simplicity Over Complexity
              </h4>
            </div>
            <p className="text-sm text-[var(--color-text-secondary)] pl-7">
              Sensible defaults, vim-style navigation, zero configuration required
            </p>
          </div>

          <div className="space-y-2">
            <div className="flex items-center gap-2">
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
              <h4 className="font-semibold text-[var(--color-text-primary)]">
                Performance Over Features
              </h4>
            </div>
            <p className="text-sm text-[var(--color-text-secondary)] pl-7">
              Native Rust binary, 5MB memory footprint, instant startup
            </p>
          </div>

          <div className="space-y-2">
            <div className="flex items-center gap-2">
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
              <h4 className="font-semibold text-[var(--color-text-primary)]">
                Community Over Profit
              </h4>
            </div>
            <p className="text-sm text-[var(--color-text-secondary)] pl-7">
              Built with community contributions, no investors, no exit strategy
            </p>
          </div>

          <div className="space-y-2">
            <div className="flex items-center gap-2">
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
              <h4 className="font-semibold text-[var(--color-text-primary)]">Safety First</h4>
            </div>
            <p className="text-sm text-[var(--color-text-secondary)] pl-7">
              100% safe Rust code, no unsafe blocks, memory-safe by design
            </p>
          </div>
        </div>

        <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
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
            <svg
              className="w-4 h-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </Link>
        </div>
      </div>
    </section>
  )
}
