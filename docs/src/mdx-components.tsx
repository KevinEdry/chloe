import type { MDXComponents } from 'mdx/types'
import { useMDXComponents as getThemeMDXComponents } from 'nextra-theme-docs'

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    ...getThemeMDXComponents(),
    ...components,
  }
}
