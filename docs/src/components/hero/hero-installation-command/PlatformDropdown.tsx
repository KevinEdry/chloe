import clsx from 'clsx'
import { type Platform, platforms } from './types'

interface PlatformDropdownProps {
  selectedPlatform: Platform
  onSelectPlatform: (platform: Platform) => void
}

export function PlatformDropdown({ selectedPlatform, onSelectPlatform }: PlatformDropdownProps) {
  return (
    <div className="relative group/dropdown">
      <div className="h-full flex items-center gap-2 px-4 py-3 text-sm font-medium bg-white text-black hover:bg-[#e5e5e5] transition-colors cursor-pointer">
        <span>Get Chloe</span>
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </div>

      <div className="hidden group-hover/dropdown:block absolute top-full left-0 pt-2 w-36 z-50">
        <div className="rounded-lg bg-[#18181b] border border-white/10 shadow-2xl">
          {(Object.keys(platforms) as Platform[]).map((platform) => (
            <button
              key={platform}
              type="button"
              onClick={() => onSelectPlatform(platform)}
              className={clsx(
                'w-full px-4 py-2.5 text-left text-sm transition-colors cursor-pointer',
                selectedPlatform === platform
                  ? 'bg-[var(--color-primary)]/20 text-[var(--color-primary-light)]'
                  : 'text-[var(--color-text-secondary)] hover:bg-white/5 hover:text-[var(--color-text-primary)]',
              )}
            >
              {platforms[platform].label}
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}
