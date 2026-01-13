'use client'

import { track } from '@vercel/analytics'
import { useState } from 'react'
import { HeroSecondaryLink } from '../HeroSecondaryLink'
import { CommandDisplay } from './CommandDisplay'
import { CopyIcon } from './CopyIcon'
import { PlatformDropdown } from './PlatformDropdown'
import { type Platform, platforms } from './types'

export function HeroInstallationCommand() {
  const [copied, setCopied] = useState(false)
  const [selectedPlatform, setSelectedPlatform] = useState<Platform>('macos')

  const handleCopy = async () => {
    const command = platforms[selectedPlatform].command
    await navigator.clipboard.writeText(command)
    track('copy_install_command', { platform: selectedPlatform })
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="pt-10 flex flex-col items-center gap-4">
      <div className="inline-flex items-stretch rounded-xl overflow-hidden border border-white/10">
        <PlatformDropdown
          selectedPlatform={selectedPlatform}
          onSelectPlatform={setSelectedPlatform}
        />

        <button
          type="button"
          onClick={handleCopy}
          className="group flex items-center gap-4 px-5 py-3 bg-white/[0.03] hover:bg-white/[0.06] transition-colors cursor-pointer"
        >
          <CommandDisplay platform={selectedPlatform} />

          <div className="flex items-center pl-2">
            <CopyIcon copied={copied} />
          </div>
        </button>
      </div>

      <HeroSecondaryLink />
    </div>
  )
}
