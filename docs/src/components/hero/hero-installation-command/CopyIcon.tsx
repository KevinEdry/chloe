interface CopyIconProps {
  copied: boolean
}

export function CopyIcon({ copied }: CopyIconProps) {
  if (copied) {
    return (
      <svg
        className="w-5 h-5 text-[var(--color-primary)]"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
      </svg>
    )
  }

  return (
    <svg
      className="w-5 h-5 text-[var(--color-text-muted)] group-hover:text-[var(--color-text-secondary)] hover:!text-[var(--color-primary-light)] transition-colors"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
      />
    </svg>
  )
}
