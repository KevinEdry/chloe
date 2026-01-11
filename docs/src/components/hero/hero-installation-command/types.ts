export type Platform = 'macos' | 'linux' | 'windows'

export const platforms: Record<Platform, { label: string; command: string }> = {
  macos: {
    label: 'macOS',
    command: 'curl -fsSL getchloe.sh/install | bash',
  },
  linux: {
    label: 'Linux',
    command: 'curl -fsSL getchloe.sh/install | bash',
  },
  windows: {
    label: 'Windows',
    command: 'irm getchloe.sh/install.ps1 | iex',
  },
}
