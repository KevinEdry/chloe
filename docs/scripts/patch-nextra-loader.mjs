import { readFileSync, writeFileSync } from 'fs'
import { fileURLToPath } from 'url'
import { join, dirname } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const loaderPath = join(__dirname, '..', 'node_modules', 'nextra', 'dist', 'server', 'loader.js')

let content
try {
  content = readFileSync(loaderPath, 'utf8')
} catch {
  process.exit(0)
}

// nextra's loader does `await import("@napi-rs/simple-git")` outside of any
// try-catch. If the native addon fails to load (which happens in the current
// Vercel build environment), the top-level await throws and `repository` stays
// in a temporal dead zone, crashing every MDX page compilation.
// Wrapping the import in try-catch makes the module degrade gracefully instead.
const broken = `const { Repository } = await import("@napi-rs/simple-git");`
const fixed = `let Repository; try { ({ Repository } = await import("@napi-rs/simple-git")); } catch { return; }`

if (!content.includes(broken)) {
  process.exit(0)
}

writeFileSync(loaderPath, content.replace(broken, fixed))
