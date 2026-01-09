import { Layout, Navbar } from 'nextra-theme-docs'
import { Footer } from '@/components/Footer'
import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import './globals.css'

export const metadata = {
  title: 'Chloe',
  description: 'Terminal UI for orchestrating parallel Claude Code instances with integrated task and terminal management',
  icons: {
    icon: '/logos/favicon.svg',
  },
}

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={<Navbar logo={<div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}><img src="/logos/logo.svg" alt="Chloe" width={36} height={36} /><span style={{ fontWeight: 700, fontSize: '1.25rem' }}>Chloe</span></div>} projectLink="https://github.com/KevinEdry/chloe" />}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/KevinEdry/chloe/tree/main/docs"
          footer={<Footer />}
        >
          {children}
        </Layout>
      </body>
    </html>
  )
}
