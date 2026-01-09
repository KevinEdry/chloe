import type { MDXComponents } from 'mdx/types'
import { useMDXComponents as getThemeMDXComponents } from 'nextra-theme-docs'

const themeComponents = getThemeMDXComponents()

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    ...themeComponents,
    ...components,
  }
}
