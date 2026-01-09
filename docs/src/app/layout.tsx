import { Footer, Layout, Navbar } from 'nextra-theme-docs'
import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import 'nextra-theme-docs/style.css'

export const metadata = {
  title: 'Chloe',
  description: 'Terminal UI for orchestrating parallel Claude Code instances with integrated task and terminal management',
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
          navbar={<Navbar logo={<span style={{ fontWeight: 700 }}>Chloe</span>} projectLink="https://github.com/KevinEdry/chloe" />}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/KevinEdry/chloe/tree/main/docs"
          footer={<Footer>MIT {new Date().getFullYear()} Chloe</Footer>}
        >
          {children}
        </Layout>
      </body>
    </html>
  )
}
