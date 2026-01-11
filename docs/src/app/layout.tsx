import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import { Layout, Navbar } from 'nextra-theme-docs'
import { Analytics } from '@vercel/analytics/next'
import { Footer } from '@/components/Footer'
import './globals.css'

const siteUrl = 'https://getchloe.sh'

export const metadata = {
  metadataBase: new URL(siteUrl),
  title: {
    default: 'Chloe - Terminal Multiplexer for Claude Code',
    template: '%s | Chloe',
  },
  description:
    'A terminal multiplexer for running parallel Claude Code instances with integrated Kanban task management. Built in 100% safe Rust.',
  keywords: [
    'Claude Code',
    'terminal multiplexer',
    'AI coding assistant',
    'Rust',
    'tmux alternative',
    'Claude',
    'Anthropic',
    'task management',
    'Kanban',
    'developer tools',
    'CLI',
  ],
  authors: [{ name: 'Kevin Edry', url: 'https://kevin-edry.com' }],
  creator: 'Kevin Edry',
  publisher: 'Kevin Edry',
  icons: {
    icon: '/logos/favicon.svg',
    apple: '/logos/logo.svg',
  },
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: siteUrl,
    siteName: 'Chloe',
    title: 'Chloe - Terminal Multiplexer for Claude Code',
    description:
      'A terminal multiplexer for running parallel Claude Code instances with integrated Kanban task management. Built in 100% safe Rust.',
    images: [
      {
        url: '/og-image.png',
        width: 1200,
        height: 630,
        alt: 'Chloe - Terminal Multiplexer for Claude Code',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Chloe - Terminal Multiplexer for Claude Code',
    description:
      'A terminal multiplexer for running parallel Claude Code instances with integrated Kanban task management. Built in 100% safe Rust.',
    images: ['/og-image.png'],
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
}

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={
            <Navbar
              logo={
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                  <img src="/logos/logo.svg" alt="Chloe" width={36} height={36} />
                  <span style={{ fontWeight: 700, fontSize: '1.25rem' }}>Chloe</span>
                </div>
              }
              projectLink="https://github.com/KevinEdry/chloe"
              chatLink="https://discord.gg/Pqdb9ZGvVV"
            />
          }
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/KevinEdry/chloe/tree/main/docs"
          footer={<Footer />}
        >
          {children}
        </Layout>
        <Analytics />
      </body>
    </html>
  )
}
