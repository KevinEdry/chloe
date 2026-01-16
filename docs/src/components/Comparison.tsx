'use client'

import { useState } from 'react'

interface Competitor {
  name: string
  type: string
  description: string
  features: {
    terminalNative: boolean | 'partial'
    kanbanBuiltIn: boolean
    multiProvider: boolean
    gitWorktrees: boolean
    memoryUsage: string
    tokenEfficient: boolean | 'partial'
    noTelemetry: boolean
    openSource: boolean
  }
}

const competitors: Competitor[] = [
  {
    name: 'Chloe',
    type: 'TUI (Terminal)',
    description: 'Terminal multiplexer built for AI coding agents',
    features: {
      terminalNative: true,
      kanbanBuiltIn: true,
      multiProvider: true,
      gitWorktrees: true,
      memoryUsage: '~5MB',
      tokenEfficient: true,
      noTelemetry: true,
      openSource: true,
    },
  },
  {
    name: 'tmux',
    type: 'TUI (Terminal)',
    description: 'General-purpose terminal multiplexer',
    features: {
      terminalNative: true,
      kanbanBuiltIn: false,
      multiProvider: false,
      gitWorktrees: false,
      memoryUsage: '~2MB',
      tokenEfficient: true,
      noTelemetry: true,
      openSource: true,
    },
  },
  {
    name: 'Auto-Claude',
    type: 'Electron Desktop',
    description: 'Multi-agent autonomous coding framework',
    features: {
      terminalNative: false,
      kanbanBuiltIn: true,
      multiProvider: true,
      gitWorktrees: true,
      memoryUsage: '~300MB',
      tokenEfficient: false,
      noTelemetry: true,
      openSource: true,
    },
  },
  {
    name: 'Vibe Kanban',
    type: 'Web (Browser)',
    description: 'Web-based kanban for AI coding agents',
    features: {
      terminalNative: false,
      kanbanBuiltIn: true,
      multiProvider: true,
      gitWorktrees: true,
      memoryUsage: '~150MB',
      tokenEfficient: true,
      noTelemetry: true,
      openSource: true,
    },
  },
  {
    name: 'Maestro',
    type: 'Electron Desktop',
    description: 'Agent orchestration command center',
    features: {
      terminalNative: false,
      kanbanBuiltIn: false,
      multiProvider: true,
      gitWorktrees: false,
      memoryUsage: '~250MB',
      tokenEfficient: 'partial',
      noTelemetry: true,
      openSource: true,
    },
  },
]

const featureLabels: { key: keyof Competitor['features']; label: string; description: string }[] = [
  {
    key: 'terminalNative',
    label: 'Terminal Native',
    description: 'Runs directly in your terminal without a browser or Electron',
  },
  {
    key: 'kanbanBuiltIn',
    label: 'Built-in Kanban',
    description: 'Integrated task management without external tools',
  },
  {
    key: 'multiProvider',
    label: 'Multi-Provider',
    description: 'Works with Claude Code, Gemini CLI, Amp, OpenCode, etc.',
  },
  {
    key: 'gitWorktrees',
    label: 'Git Worktrees',
    description: 'Native support for parallel branch development',
  },
  {
    key: 'memoryUsage',
    label: 'Memory Usage',
    description: 'Typical RAM consumption during operation',
  },
  {
    key: 'tokenEfficient',
    label: 'Token Efficient',
    description: 'No extra prompting that burns API tokens',
  },
  {
    key: 'noTelemetry',
    label: 'No Telemetry',
    description: 'No data collection or phone-home behavior',
  },
  {
    key: 'openSource',
    label: 'Open Source',
    description: 'Source code is publicly available',
  },
]

function FeatureValue({ value }: { value: boolean | string }) {
  if (value === 'partial') {
    return (
      <div className="w-5 h-5 rounded-full bg-amber-500/20 flex items-center justify-center">
        <span className="text-amber-400 text-xs font-bold">~</span>
      </div>
    )
  }

  if (typeof value === 'string') {
    return <span className="text-[var(--color-text-secondary)] text-sm">{value}</span>
  }

  if (value === true) {
    return (
      <div className="w-5 h-5 rounded-full bg-emerald-500/20 flex items-center justify-center">
        <svg className="w-3 h-3 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
        </svg>
      </div>
    )
  }

  return (
    <div className="w-5 h-5 rounded-full bg-[var(--color-surface)] flex items-center justify-center border border-[var(--color-border)]">
      <svg className="w-3 h-3 text-[var(--color-text-tertiary)]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 12H4" />
      </svg>
    </div>
  )
}

const chloe = competitors[0]
const otherCompetitors = competitors.slice(1)

export function Comparison() {
  const [selectedCompetitor, setSelectedCompetitor] = useState(0)
  const currentCompetitor = otherCompetitors[selectedCompetitor]

  return (
    <section className="relative" id="comparison">
      <div className="max-w-[1208px] mx-auto px-6 py-20">
        {/* Header */}
        <div className="max-w-3xl mb-12">
          <h2 className="text-3xl md:text-4xl font-semibold text-[var(--color-text-primary)] mb-4">
            How Chloe Compares
          </h2>
          <p className="text-lg text-[var(--color-text-secondary)] leading-relaxed">
            See how Chloe stacks up against other tools for managing AI coding agents.
          </p>
        </div>

        {/* Mobile: Card comparison */}
        <div className="lg:hidden">
          {/* Competitor selector */}
          <div className="flex flex-wrap gap-2 mb-6">
            {otherCompetitors.map((competitor, index) => (
              <button
                key={competitor.name}
                type="button"
                onClick={() => setSelectedCompetitor(index)}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-all ${
                  selectedCompetitor === index
                    ? 'bg-[var(--color-primary)] text-white'
                    : 'bg-[var(--color-surface)] text-[var(--color-text-secondary)] border border-[var(--color-border)]'
                }`}
              >
                {competitor.name}
              </button>
            ))}
          </div>

          {/* Comparison card */}
          <div className="bg-[var(--color-surface)]/30 border border-[var(--color-border)] rounded-xl overflow-hidden">
            {/* Header */}
            <div className="grid grid-cols-3 border-b border-[var(--color-border)]">
              <div className="p-4" />
              <div className="p-4 text-center bg-[var(--color-primary)]/10 border-x border-[var(--color-primary)]/20">
                <div className="font-semibold text-[var(--color-primary-light)]">{chloe.name}</div>
                <div className="text-xs text-[var(--color-text-tertiary)]">{chloe.type}</div>
              </div>
              <div className="p-4 text-center">
                <div className="font-semibold text-[var(--color-text-primary)]">{currentCompetitor.name}</div>
                <div className="text-xs text-[var(--color-text-tertiary)]">{currentCompetitor.type}</div>
              </div>
            </div>

            {/* Features */}
            {featureLabels.map((feature, index) => (
              <div
                key={feature.key}
                className={`grid grid-cols-3 items-center ${
                  index !== featureLabels.length - 1 ? 'border-b border-[var(--color-border)]' : ''
                }`}
              >
                <div className="p-3 text-sm text-[var(--color-text-primary)]">{feature.label}</div>
                <div className="p-3 flex justify-center bg-[var(--color-primary)]/5 border-x border-[var(--color-primary)]/10">
                  <FeatureValue value={chloe.features[feature.key]} />
                </div>
                <div className="p-3 flex justify-center">
                  <FeatureValue value={currentCompetitor.features[feature.key]} />
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Desktop: Full table */}
        <div className="hidden lg:block">
          <div className="bg-[var(--color-surface)]/30 border border-[var(--color-border)] rounded-xl overflow-hidden">
            {/* Header Row */}
            <div className="grid grid-cols-6 border-b border-[var(--color-border)]">
              <div className="p-4" />
              {competitors.map((competitor, index) => (
                <div
                  key={competitor.name}
                  className={`p-4 text-center ${
                    index === 0
                      ? 'bg-[var(--color-primary)]/10 border-x border-[var(--color-primary)]/20'
                      : ''
                  }`}
                >
                  <div
                    className={`font-semibold ${index === 0 ? 'text-[var(--color-primary-light)]' : 'text-[var(--color-text-primary)]'}`}
                  >
                    {competitor.name}
                  </div>
                  <div className="text-xs text-[var(--color-text-tertiary)] mt-1">{competitor.type}</div>
                </div>
              ))}
            </div>

            {/* Feature Rows */}
            {featureLabels.map((feature, featureIndex) => (
              <div
                key={feature.key}
                className={`grid grid-cols-6 items-center ${
                  featureIndex !== featureLabels.length - 1 ? 'border-b border-[var(--color-border)]' : ''
                }`}
              >
                <div className="p-4">
                  <div className="text-sm font-medium text-[var(--color-text-primary)]">{feature.label}</div>
                </div>

                {competitors.map((competitor, index) => (
                  <div
                    key={`${competitor.name}-${feature.key}`}
                    className={`p-4 flex justify-center ${
                      index === 0 ? 'bg-[var(--color-primary)]/5 border-x border-[var(--color-primary)]/10' : ''
                    }`}
                  >
                    <FeatureValue value={competitor.features[feature.key]} />
                  </div>
                ))}
              </div>
            ))}
          </div>
        </div>

        {/* Bottom note */}
        <p className="text-sm text-[var(--color-text-tertiary)] mt-6 text-center">
          All tools listed are open source. Memory usage is approximate and varies by workload.
        </p>
      </div>
    </section>
  )
}
