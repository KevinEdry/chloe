/** @type {import('next-sitemap').IConfig} */
export default {
  siteUrl: 'https://getchloe.sh',
  generateRobotsTxt: true,
  generateIndexSitemap: false,
  outDir: 'out',
  robotsTxtOptions: {
    policies: [
      {
        userAgent: '*',
        allow: '/',
      },
    ],
  },
}
