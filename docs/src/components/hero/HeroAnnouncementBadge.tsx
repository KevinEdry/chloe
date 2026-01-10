import Link from 'next/link'

export function HeroAnnouncementBadge() {
  return (
    <div className="pb-8">
      <Link
        href="/docs/overview"
        className="inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm bg-[var(--color-primary)]/10 border border-[var(--color-primary)]/20 text-[var(--color-primary-light)] hover:bg-[var(--color-primary)]/15 hover:border-[var(--color-primary)]/30 transition-all"
      >
        <span>100% Safe Rust</span>
        <span className="opacity-60">·</span>
        <span>Vim-style navigation</span>
        <span className="ml-1">→</span>
      </Link>
    </div>
  )
}
