export function HeroStats() {
  return (
    <div className="mt-8 flex items-center justify-center gap-6 text-sm text-[var(--color-text-muted)]">
      <a
        href="https://github.com/KevinEdry/chloe"
        target="_blank"
        rel="noopener noreferrer"
        className="inline-flex items-center gap-1.5 hover:text-[var(--color-text-secondary)] transition-colors"
      >
        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z" />
        </svg>
        Star on GitHub
      </a>
      <span className="opacity-40">·</span>
      <span>Built with Rust</span>
    </div>
  )
}
