import { Platform } from './types'

interface CommandDisplayProps {
  platform: Platform
}

export function CommandDisplay({ platform }: CommandDisplayProps) {
  if (platform === 'windows') {
    return (
      <code className="font-mono text-base">
        <span className="text-[var(--color-primary-light)]">irm</span>
        <span className="text-[var(--color-text-muted)]"> </span>
        <span className="text-white">getchloe.sh/install.ps1</span>
        <span className="text-sky-400"> | iex</span>
      </code>
    )
  }

  return (
    <code className="font-mono text-base">
      <span className="text-[var(--color-primary-light)]">curl -fsSL</span>
      <span className="text-[var(--color-text-muted)]"> </span>
      <span className="text-white">getchloe.sh/install</span>
      <span className="text-sky-400"> | bash</span>
    </code>
  )
}
