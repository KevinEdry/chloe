import type { ReactNode } from 'react'
import Link from 'next/link'
import { FaqItem } from './FaqItem'

interface FaqItemData {
  question: string
  answer: string | ReactNode
}

const faqItems: FaqItemData[] = [
  {
    question: 'What is Chloe?',
    answer:
      'Chloe is a terminal-based application built in Rust that helps you manage multiple AI coding agents simultaneously. It works with Claude Code, Gemini CLI, Amp, OpenCode, and any terminal-based AI tool. It combines a Kanban board for task management with a terminal multiplexer for running parallel coding sessions.',
  },
  {
    question: 'How is Chloe different from tmux or screen?',
    answer: (
      <>
        While tmux and screen are general-purpose terminal multiplexers, Chloe is specifically
        designed for AI-assisted coding workflows. It integrates task management directly with your
        terminal sessions, so you can associate tasks with specific AI agent instances and track
        progress visually.{' '}
        <Link
          href="/comparisons/chloe-vs-tmux-screen"
          className="text-[var(--color-primary-light)] hover:underline"
        >
          Read the full comparison guide
        </Link>
        .
      </>
    ),
  },
  {
    question: 'Does Chloe require an API key?',
    answer:
      'No, Chloe itself does not require an API key. It manages terminal sessions where you run your AI coding agents. Each provider (Claude Code, Gemini CLI, etc.) handles its own authentication separately.',
  },
  {
    question: 'What platforms does Chloe support?',
    answer:
      'Chloe runs on macOS, Linux, and Windows. The installation script automatically detects your platform and installs the appropriate binary.',
  },
  {
    question: 'Is Chloe open source?',
    answer:
      'Yes, Chloe is fully open source and available on GitHub. It is written in 100% safe Rust with no unsafe code blocks, making it memory-safe and secure by design.',
  },
  {
    question: 'Which AI coding agents does Chloe support?',
    answer:
      'Chloe has built-in support for Claude Code, Gemini CLI, Amp, and OpenCode. It auto-detects installed providers and lets you choose which one to use for each task. You can also use any other terminal-based AI tool.',
  },
  {
    question: 'Can I use Chloe with Cursor or VS Code?',
    answer:
      "Yes! Chloe runs in any terminal, so you can use it alongside Cursor, VS Code, or any other editor. Open Chloe in your editor's integrated terminal or run it in a separate terminal window. Many developers use Chloe to manage multiple AI agents while their editor handles code editing.",
  },
  {
    question: 'How do I migrate from tmux to Chloe?',
    answer: (
      <>
        Migrating from tmux is straightforward. Install Chloe, start it in your project directory,
        and use familiar vim-style navigation (hjkl). Create tasks with{' '}
        <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
          n
        </kbd>
        , split panes with{' '}
        <kbd className="px-1.5 py-0.5 text-xs rounded bg-[var(--color-surface)] border border-[var(--color-border)]">
          s
        </kbd>
        . You can run Chloe inside tmux if you want session persistence for remote servers.{' '}
        <Link
          href="/comparisons/chloe-vs-tmux-screen"
          className="text-[var(--color-primary-light)] hover:underline"
        >
          See the full migration guide
        </Link>
        .
      </>
    ),
  },
  {
    question: 'Can I use Chloe for non-AI terminal work?',
    answer:
      "Absolutely! While Chloe is optimized for AI coding workflows, it's a general-purpose terminal multiplexer. Use it to run build processes, tests, servers, or any terminal-based tasks. The Kanban board helps organize any type of work, not just AI-assisted coding.",
  },
  {
    question: 'How much memory does Chloe use?',
    answer:
      'Chloe uses approximately 5MB of RAM, compared to 100-500MB for Electron-based alternatives. The native Rust binary is extremely lightweight and starts instantly. For comparison, tmux uses ~2MB and screen uses ~1MB.',
  },
  {
    question: 'Does Chloe support custom themes or configuration?',
    answer:
      "Chloe ships with sensible defaults and doesn't currently require configuration files. Theme support and customization options are on the roadmap. The goal is to keep configuration optional while providing power users with flexibility when needed.",
  },
  {
    question: 'How do I update Chloe to the latest version?',
    answer:
      'Run the installation script again: curl -fsSL https://getchloe.sh/install.sh | sh. This will download and install the latest version. Your tasks and settings are preserved during updates.',
  },
  {
    question: 'Is Chloe faster than tmux?',
    answer:
      'Both Chloe and tmux are fast enough that you won\'t notice a performance difference in normal use. Chloe uses slightly more memory (~5MB vs ~2MB) due to its built-in task management features, but startup time and responsiveness are comparable. The main difference is ease of use, not raw performance.',
  },
  {
    question: 'Can I contribute to Chloe development?',
    answer: (
      <>
        Yes! Chloe is open source and welcomes contributions. Visit the{' '}
        <Link
          href="https://github.com/kevinedry/chloe"
          target="_blank"
          rel="noopener noreferrer"
          className="text-[var(--color-primary-light)] hover:underline"
        >
          GitHub repository
        </Link>{' '}
        to report issues, suggest features, or submit pull requests. Check the CONTRIBUTING.md file
        for guidelines.
      </>
    ),
  },
  {
    question: 'Does Chloe collect any data or phone home?',
    answer:
      'No. Chloe does not collect telemetry, usage data, or any personal information. There are no network requests, no tracking, no analytics. Your work stays completely private on your machine. You can verify this by inspecting the open source code.',
  },
  {
    question: "What's the difference between Git worktrees and Jujutsu workspaces in Chloe?",
    answer:
      'Both let you work on multiple branches simultaneously. Git worktrees create separate working directories for different branches of the same repository. Jujutsu workspaces provide similar functionality with a different approach to version control. Chloe auto-detects which system you\'re using and provides appropriate UI indicators.',
  },
]

const faqSchema = {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  mainEntity: faqItems.map((item) => {
    let answerText = ''
    if (typeof item.answer === 'string') {
      answerText = item.answer
    } else {
      switch (item.question) {
        case 'How is Chloe different from tmux or screen?':
          answerText =
            'While tmux and screen are general-purpose terminal multiplexers, Chloe is specifically designed for AI-assisted coding workflows. It integrates task management directly with your terminal sessions, so you can associate tasks with specific AI agent instances and track progress visually.'
          break
        case 'How do I migrate from tmux to Chloe?':
          answerText =
            "Migrating from tmux is straightforward. Install Chloe, start it in your project directory, and use familiar vim-style navigation. Create tasks with 'n', split panes with 's'. You can run Chloe inside tmux if you want session persistence for remote servers."
          break
        case 'Can I contribute to Chloe development?':
          answerText =
            'Yes! Chloe is open source and welcomes contributions. Visit the GitHub repository to report issues, suggest features, or submit pull requests.'
          break
        default:
          answerText = item.question
      }
    }

    return {
      '@type': 'Question',
      name: item.question,
      acceptedAnswer: {
        '@type': 'Answer',
        text: answerText,
      },
    }
  }),
}

export function Faq() {
  return (
    <section className="relative bg-[var(--color-surface)] border-y border-[var(--color-border)]">
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(faqSchema) }}
      />
      <div className="px-6 py-20 flex flex-col items-center">
        <h2 className="text-2xl font-semibold text-[var(--color-text-primary)] pb-8">
          Frequently Asked Questions
        </h2>

        <div className="w-full max-w-2xl">
          {faqItems.map((item) => (
            <FaqItem key={item.question} question={item.question} answer={item.answer} />
          ))}
        </div>
      </div>
    </section>
  )
}
