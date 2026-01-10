import Link from 'next/link'

export function HeroSecondaryLink() {
  return (
    <div className="flex items-center gap-1 text-sm text-[var(--color-text-muted)]">
      <span>Or read the</span>
      <Link
        href="/docs/quick-start"
        className="text-[var(--color-primary-light)] hover:text-[var(--color-primary)] underline underline-offset-2 decoration-[var(--color-primary-light)]/30 hover:decoration-[var(--color-primary)]/50 transition-colors"
      >
        documentation
      </Link>
    </div>
  )
}
