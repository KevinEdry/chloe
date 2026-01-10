import { HeroOverlay } from './HeroOverlay'
import { HeroTiledBackground } from './HeroTiledBackground'
import { HeroContent } from './HeroContent'

export function Hero() {
  return (
    <section className="relative overflow-hidden">
      <HeroOverlay />
      <HeroTiledBackground />
      <HeroContent />
    </section>
  )
}
