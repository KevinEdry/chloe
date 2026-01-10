import Image from 'next/image'
import Link from 'next/link'

export function Footer() {
  return (
    <footer className="w-full flex justify-center items-center flex-col gap-10 py-10">
      <div className="flex gap-5 w-full justify-center items-center">
        <div className="w-1/3 h-px bg-[var(--color-border)]"></div>
        <Link href="/">
          <div className="min-w-[75px]">
            <Image
              className="hover:scale-105 transition-all rounded-2xl hover:drop-shadow-[0_0_8px_rgba(168,85,247,0.25)]"
              src="/logos/logo.svg"
              alt="Chloe logo"
              width={75}
              height={75}
            />
          </div>
        </Link>
        <div className="w-1/3 h-px bg-[var(--color-border)]"></div>
      </div>
      <div className="flex flex-col gap-1 items-center">
        <ul className="flex gap-5 w-full justify-center text-[var(--color-text-secondary)] font-medium [&_li]:cursor-pointer [&_li:hover]:text-[var(--color-text-muted)]">
          <li>
            <Link href="/docs/overview">Docs</Link>
          </li>
          <li>
            <Link href="https://github.com/KevinEdry/chloe" target="_blank">
              GitHub
            </Link>
          </li>
        </ul>
        <div className="text-[var(--color-text-muted)] text-sm mt-4">
          Made with ❤️ in Seattle by{' '}
          <Link
            href="https://kevin-edry.com/"
            className="hover:text-[var(--color-primary)] transition-all font-bold"
            target="_blank"
          >
            Kevin Edry
          </Link>
          .
        </div>
      </div>
    </footer>
  )
}
