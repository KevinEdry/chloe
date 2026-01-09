import Link from 'next/link'

export function Demo() {
  return (
    <section className="relative px-6 pt-0 pb-12 -translate-y-32 mb-[-8rem]">
      <div className="mx-auto max-w-5xl">
        <Link
          href="/docs"
          className="block group"
        >
          <div className="relative rounded-xl overflow-hidden border border-[var(--color-border)] shadow-2xl shadow-black/50 transition-all duration-300 group-hover:border-[var(--color-border-light)] group-hover:shadow-[0_25px_50px_-12px_rgba(0,0,0,0.7),0_0_60px_rgba(168,85,247,0.15)] bg-[var(--color-surface)]">
            {/* Terminal header */}
            <div className="flex items-center gap-2 px-4 py-3 bg-[var(--color-surface-elevated)] border-b border-[var(--color-border)]">
              <div className="w-3 h-3 rounded-full bg-[#ff5f57]" />
              <div className="w-3 h-3 rounded-full bg-[#febc2e]" />
              <div className="w-3 h-3 rounded-full bg-[#28c840]" />
              <span className="ml-4 text-sm text-[var(--color-text-muted)]">chloe</span>
            </div>

            {/* Terminal content - Demo Video */}
            <div className="relative bg-black">
              <video
                autoPlay
                loop
                muted
                playsInline
                className="w-full h-auto"
              >
                <source src="/demo.webm" type="video/webm" />
                <source src="/demo.gif" type="image/gif" />
              </video>
            </div>

            {/* Hover overlay */}
            <div className="absolute inset-0 bg-[var(--color-primary)]/0 group-hover:bg-[var(--color-primary)]/5 transition-colors duration-300 flex items-center justify-center opacity-0 group-hover:opacity-100">
              <span className="px-4 py-2 rounded-lg bg-[var(--color-surface)] border border-[var(--color-border)] text-sm font-medium text-[var(--color-text-primary)]">
                View documentation →
              </span>
            </div>
          </div>
        </Link>
      </div>
    </section>
  )
}
