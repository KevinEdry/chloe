'use client'

import type { ReactNode } from 'react'
import { useState } from 'react'

interface FaqItemProps {
  question: string
  answer: string | ReactNode
}

export function FaqItem({ question, answer }: FaqItemProps) {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <div className="border-b border-[var(--color-border)]">
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="w-full py-5 flex items-center justify-between text-left cursor-pointer"
      >
        <span className="text-[var(--color-text-primary)] font-medium">{question}</span>
        <svg
          className={`w-5 h-5 text-[var(--color-text-muted)] transition-transform ${isOpen ? 'rotate-180' : ''}`}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {isOpen && (
        <div className="pb-5 text-[var(--color-text-secondary)] leading-relaxed">{answer}</div>
      )}
    </div>
  )
}
