import type { ReactNode } from 'react'

interface FeatureProps {
  icon: ReactNode
  title: string
  description: ReactNode
}

function Feature({ icon, title, description }: FeatureProps) {
  return (
    <li className="max-w-[300px] sm:max-w-[256px] text-[var(--color-text-secondary)] leading-relaxed">
      <span className="inline-flex items-center align-middle mr-2 text-[var(--color-primary-light)]">
        {icon}
      </span>
      <strong className="text-[var(--color-text-primary)] font-semibold">{title}</strong>
      <span className="hidden sm:inline">. </span>
      <span className="block sm:inline">{description}</span>
    </li>
  )
}

export function Features() {
  return (
    <section className="relative max-w-[1208px] mx-auto px-6 py-20">
      <h2 className="text-2xl font-semibold text-[var(--color-text-primary)] pb-12">Features</h2>

      {/* Features grid */}
      <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10 list-none">
        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"
              />
            </svg>
          }
          title="Kanban Board"
          description="Drag-free task management with To Do, In Progress, and Done columns"
        />

        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
          }
          title="Multi-Provider Support"
          description="Works with Claude Code, Gemini CLI, Amp, OpenCode, and any terminal-based AI agent"
        />

        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
          }
          title="Vim Navigation"
          description={
            <>
              Navigate with{' '}
              <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
                h
              </kbd>{' '}
              <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
                j
              </kbd>{' '}
              <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
                k
              </kbd>{' '}
              <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
                l
              </kbd>{' '}
              or arrow keys
            </>
          }
        />

        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2"
              />
            </svg>
          }
          title="Git Worktrees"
          description="Manage git worktrees for parallel branch development"
        />

        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
          }
          title="Roadmap View"
          description="Plan and visualize project milestones with timeline tracking"
        />

        <Feature
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
              />
            </svg>
          }
          title="Memory Safe"
          description="100% safe Rust with zero unsafe code blocks"
        />
      </ul>
    </section>
  )
}
