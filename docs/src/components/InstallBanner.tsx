'use client'

import { useState } from 'react'

const INSTALL_COMMAND = 'curl -fsSL getchloe.sh/install | bash'

export function InstallBanner() {
  const [copied, setCopied] = useState(false)

  const handleCopy = async () => {
    await navigator.clipboard.writeText(INSTALL_COMMAND)
    setCopied(true)
    setTimeout(() => setCopied(false), 1500)
  }

  return (
    <div className="border-y border-[var(--color-border)] bg-[var(--color-surface)]/30">
      <div className="max-w-[1208px] mx-auto px-6 py-16 flex flex-col items-center text-center">
        <h3 className="text-2xl font-semibold text-[var(--color-text-primary)] mb-2">
          Ready to try Chloe?
        </h3>
        <p className="text-[var(--color-text-secondary)] mb-8">
          Install with a single command and start shipping faster.
        </p>

        {/* Command box */}
        <div className="flex items-center bg-[#0d1117] rounded-lg border border-[#30363d] overflow-hidden">
          <div className="flex items-center gap-3 px-5 py-4 font-mono text-base">
            <span className="text-[#6e7681]">$</span>
            <code>
              <span className="text-[var(--color-primary-light)]">curl -fsSL</span>
              <span className="text-white"> getchloe.sh/install</span>
              <span className="text-sky-400"> | bash</span>
            </code>
          </div>
          <button
            type="button"
            onClick={handleCopy}
            className="flex items-center justify-center h-full px-4 py-4 border-l border-[#30363d] hover:bg-[#161b22] transition-colors"
            title={copied ? 'Copied!' : 'Copy to clipboard'}
          >
            {copied ? (
              <svg
                className="w-5 h-5 text-[#27c93f]"
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
            ) : (
              <svg
                className="w-5 h-5 text-[#8b949e] hover:text-[#e6edf3]"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                />
              </svg>
            )}
          </button>
        </div>
      </div>
    </div>
  )
}
