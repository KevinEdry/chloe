import { HeroAnnouncementBadge } from './HeroAnnouncementBadge'
import { HeroHeadline } from './HeroHeadline'
import { HeroStats } from './HeroStats'
import { HeroSubtitle } from './HeroSubtitle'
import { HeroInstallationCommand } from './hero-installation-command/HeroInstallationCommand'

export function HeroContent() {
  return (
    <div className="relative z-[2] px-6 pt-16 pb-36 md:pt-24 md:pb-44 flex justify-center items-center">
      <div className="max-w-3xl text-center flex flex-col items-center">
        <HeroAnnouncementBadge />
        <HeroHeadline />
        <HeroSubtitle />
        <HeroInstallationCommand />
        <HeroStats />
      </div>
    </div>
  )
}
