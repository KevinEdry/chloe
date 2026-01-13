import { HeroContent } from './HeroContent'
import { HeroOverlay } from './HeroOverlay'
import { HeroTiledBackground } from './HeroTiledBackground'

export function Hero() {
  return (
    <section className="relative overflow-hidden">
      <HeroOverlay />
      <HeroTiledBackground />
      <HeroContent />
    </section>
  )
}
