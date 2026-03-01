import { readFileSync, writeFileSync } from 'fs'
import { fileURLToPath } from 'url'
import { join, dirname } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const loaderPath = join(__dirname, '..', 'node_modules', 'nextra', 'dist', 'server', 'loader.js')

console.log('[nextra-patch] Target:', loaderPath)

let content
try {
  content = readFileSync(loaderPath, 'utf8')
  console.log('[nextra-patch] File found, size:', content.length)
} catch {
  console.log('[nextra-patch] File not found — install not complete, skipping')
  process.exit(0)
}

let changed = false

// Strategy 1: wrap the bare `await import(...)` in try-catch so a failed native
// addon load doesn't leave `repository` in the temporal dead zone.
const importBroken = `const { Repository } = await import("@napi-rs/simple-git");`
const importFixed  = `let Repository; try { ({ Repository } = await import("@napi-rs/simple-git")); } catch { return; }`

if (content.includes(importBroken)) {
  content = content.replace(importBroken, importFixed)
  changed = true
  console.log('[nextra-patch] Strategy 1 applied (import try-catch)')
} else {
  console.log('[nextra-patch] Strategy 1 string not found')
  // Log the context around the package name to help diagnose
  const index = content.indexOf('@napi-rs/simple-git')
  if (index !== -1) {
    const snippet = content.substring(Math.max(0, index - 40), index + 80)
    console.log('[nextra-patch] Context around @napi-rs/simple-git:', JSON.stringify(snippet))
  } else {
    console.log('[nextra-patch] @napi-rs/simple-git not found in file at all')
  }
}

// Strategy 2: wrap the getLastCommitTime() call in .catch() so a TDZ reference
// error (when strategy 1 did not apply) becomes a graceful undefined instead of
// crashing the webpack compilation.
const callBroken = `await getLastCommitTime(resourcePath) : NOW;`
const callFixed  = `await getLastCommitTime(resourcePath).catch(() => void 0) : NOW;`

if (content.includes(callBroken)) {
  content = content.replace(callBroken, callFixed)
  changed = true
  console.log('[nextra-patch] Strategy 2 applied (call-site catch)')
} else {
  console.log('[nextra-patch] Strategy 2 string not found (may already be patched)')
}

if (!changed) {
  console.log('[nextra-patch] Nothing to patch')
  process.exit(0)
}

writeFileSync(loaderPath, content)
console.log('[nextra-patch] Done')
