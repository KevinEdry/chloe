import { FaqItem } from './FaqItem'

interface FaqItemData {
  question: string
  answer: string
}

const faqItems: FaqItemData[] = [
  {
    question: 'What is Chloe?',
    answer:
      'Chloe is a terminal-based application built in Rust that helps you manage multiple Claude Code instances simultaneously. It combines a Kanban board for task management with a terminal multiplexer for running parallel coding sessions.',
  },
  {
    question: 'How is Chloe different from tmux or screen?',
    answer:
      'While tmux and screen are general-purpose terminal multiplexers, Chloe is specifically designed for AI-assisted coding workflows. It integrates task management directly with your terminal sessions, so you can associate tasks with specific Claude Code instances and track progress visually.',
  },
  {
    question: 'Does Chloe require an Anthropic API key?',
    answer:
      'No, Chloe itself does not require an API key. It manages terminal sessions where you run Claude Code. Claude Code handles its own authentication separately.',
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
    question: 'Can I use Chloe with other AI coding assistants?',
    answer:
      'While Chloe is optimized for Claude Code workflows, the terminal multiplexer can run any command-line application. You can use it with any terminal-based tool or AI assistant.',
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
