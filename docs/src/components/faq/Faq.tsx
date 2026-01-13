import { FaqItem } from './FaqItem'

interface FaqItemData {
  question: string
  answer: string
}

const faqItems: FaqItemData[] = [
  {
    question: 'What is Chloe?',
    answer:
      'Chloe is a terminal-based application built in Rust that helps you manage multiple AI coding agents simultaneously. It works with Claude Code, Gemini CLI, Amp, OpenCode, and any terminal-based AI tool. It combines a Kanban board for task management with a terminal multiplexer for running parallel coding sessions.',
  },
  {
    question: 'How is Chloe different from tmux or screen?',
    answer:
      'While tmux and screen are general-purpose terminal multiplexers, Chloe is specifically designed for AI-assisted coding workflows. It integrates task management directly with your terminal sessions, so you can associate tasks with specific AI agent instances and track progress visually.',
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
]

const faqSchema = {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  mainEntity: faqItems.map((item) => ({
    '@type': 'Question',
    name: item.question,
    acceptedAnswer: {
      '@type': 'Answer',
      text: item.answer,
    },
  })),
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
