import nextra from 'nextra'

const withNextra = nextra({
  gitTimestamp: false,
})

export default withNextra({
  output: 'export',
  images: {
    unoptimized: true,
  },
})
