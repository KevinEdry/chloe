'use client'

import type { ReactNode } from 'react'
import { useState } from 'react'

interface FaqItemProps {
  question: string
  answer: string | ReactNode
  isLast?: boolean
}

export function FaqItem({ question, answer, isLast = false }: FaqItemProps) {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <div className={`${!isLast ? 'border-b border-[var(--color-border)]' : ''}`}>
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-6 py-5 flex items-center justify-between text-left cursor-pointer hover:bg-[var(--color-surface)]/50 transition-colors"
      >
        <span className="text-[var(--color-text-primary)] font-medium pr-4">{question}</span>
        <div
          className={`flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center transition-all ${
            isOpen
              ? 'bg-[var(--color-primary)] text-white rotate-45'
              : 'bg-[var(--color-surface)] text-[var(--color-text-secondary)] border border-[var(--color-border)]'
          }`}
        >
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
        </div>
      </button>
      <div
        className={`overflow-hidden transition-all duration-300 ease-in-out ${
          isOpen ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'
        }`}
      >
        <div className="px-6 pb-5 text-[var(--color-text-secondary)] leading-relaxed">{answer}</div>
      </div>
    </div>
  )
}
