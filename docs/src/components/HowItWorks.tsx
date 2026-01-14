interface StepProps {
  number: number
  title: string
  description: string
}

function Step({ number, title, description }: StepProps) {
  return (
    <li className="flex gap-6">
      <div className="flex-shrink-0">
        <div className="flex items-center justify-center w-10 h-10 rounded-full bg-[var(--color-primary)]/10 border border-[var(--color-primary)]/20 text-[var(--color-primary-light)] font-bold">
          {number}
        </div>
      </div>
      <div className="flex-1 pt-1">
        <h3 className="text-lg font-semibold text-[var(--color-text-primary)] mb-2">{title}</h3>
        <p className="text-[var(--color-text-secondary)] leading-relaxed">{description}</p>
      </div>
    </li>
  )
}

export function HowItWorks() {
  return (
    <section className="relative bg-[var(--color-surface)]/30 border-y border-[var(--color-border)]">
      <div className="max-w-[1208px] mx-auto px-6 py-20">
        <h2 className="text-2xl font-semibold text-[var(--color-text-primary)] pb-12">
          How It Works
        </h2>

        <ul className="grid grid-cols-1 md:grid-cols-2 gap-10 list-none">
          <Step
            number={1}
            title="Install in one command"
            description="Get started instantly with a simple curl command. No configuration files, no complex setup, no credit card required."
          />

          <Step
            number={2}
            title="Create tasks on the Kanban board"
            description="Organize your work with a built-in Kanban board. Move tasks between To Do, In Progress, and Done columns with vim-style navigation."
          />

          <Step
            number={3}
            title="Launch AI agent instances"
            description="Run Claude Code, Gemini CLI, Amp, OpenCode, and other AI coding agents in parallel. Chloe automatically detects and configures each agent."
          />

          <Step
            number={4}
            title="Work across multiple branches"
            description="Leverage Git worktrees or Jujutsu workspaces to work on multiple branches simultaneously. Each instance can run in a different branch."
          />

          <Step
            number={5}
            title="Track progress visually"
            description="See all your agents working in real-time. Monitor task status, pause and resume agents, and manage your workflow from a single terminal."
          />

          <Step
            number={6}
            title="Ship faster with parallel work"
            description="Complete multiple features simultaneously, review PRs while building new ones, and maintain context across all your work streams."
          />
        </ul>
      </div>
    </section>
  )
}
