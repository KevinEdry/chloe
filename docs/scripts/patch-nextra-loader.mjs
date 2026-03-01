import { readFileSync, writeFileSync } from 'fs'
import { fileURLToPath } from 'url'
import { join, dirname } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const loaderJsPath = join(__dirname, '..', 'node_modules', 'nextra', 'dist', 'server', 'loader.js')
const loaderCjsPath = join(__dirname, '..', 'node_modules', 'nextra', 'loader.cjs')

// ── loader.js patches ────────────────────────────────────────────────────────

let jsContent
try {
  jsContent = readFileSync(loaderJsPath, 'utf8')
  console.log('[nextra-patch] loader.js found, size:', jsContent.length)
} catch {
  console.log('[nextra-patch] loader.js not found — skipping')
  process.exit(0)
}

let jsChanged = false

// Strategy 1: wrap the bare `await import(...)` in try-catch so a failed
// native addon load doesn't crash the module.
const importBroken = `const { Repository } = await import("@napi-rs/simple-git");`
const importFixed  = `let Repository; try { ({ Repository } = await import("@napi-rs/simple-git")); } catch { return; }`

if (jsContent.includes(importBroken)) {
  jsContent = jsContent.replace(importBroken, importFixed)
  jsChanged = true
  console.log('[nextra-patch] Strategy 1 applied (import try-catch)')
} else {
  console.log('[nextra-patch] Strategy 1 already applied or not found')
}

// Strategy 2: wrap the getLastCommitTime() call in .catch() so any error
// there becomes a graceful undefined instead of crashing the build.
const callBroken = `await getLastCommitTime(resourcePath) : NOW;`
const callFixed  = `await getLastCommitTime(resourcePath).catch(() => void 0) : NOW;`

if (jsContent.includes(callBroken)) {
  jsContent = jsContent.replace(callBroken, callFixed)
  jsChanged = true
  console.log('[nextra-patch] Strategy 2 applied (call-site catch)')
} else {
  console.log('[nextra-patch] Strategy 2 already applied or not found')
}

if (jsChanged) {
  writeFileSync(loaderJsPath, jsContent)
  console.log('[nextra-patch] loader.js written')
}

// ── loader.cjs patch ─────────────────────────────────────────────────────────
//
// Bun resolves import() before the ESM module's code starts executing, so
// concurrent webpack workers can call loader() while const bindings in
// loader.js are still in TDZ. loader.cjs is a CJS module whose variables are
// always initialized, so we use module-level variables to serialise the import
// and retry on TDZ ReferenceErrors until the ESM module finishes initializing.

let cjsContent
try {
  cjsContent = readFileSync(loaderCjsPath, 'utf8')
  console.log('[nextra-patch] loader.cjs found, size:', cjsContent.length)
} catch {
  console.log('[nextra-patch] loader.cjs not found — skipping')
  process.exit(0)
}

const cjsBroken = `module.exports = async function loader(code) {
  const callback = this.async()

  try {
    // Note that \`import()\` caches, so this should be fast enough.
    const { loader } = await import('./dist/server/loader.js')
    const result = await loader.call(this, code)
    callback(null, result)
  } catch (error) {
    callback(error)
  }
}`

const cjsFixed = `let _importPromise = null

module.exports = async function loader(code) {
  const callback = this.async()

  try {
    if (!_importPromise) {
      _importPromise = import('./dist/server/loader.js')
    }
    const mod = await _importPromise

    // Bun can resolve import() before the ESM module's synchronous code runs,
    // leaving const bindings in TDZ. Retry with backoff until the module
    // finishes initializing.
    for (let attempt = 0; attempt < 10; attempt++) {
      try {
        const result = await mod.loader.call(this, code)
        callback(null, result)
        return
      } catch (err) {
        if (err instanceof ReferenceError && attempt < 9) {
          await new Promise(r => setTimeout(r, 10 * (attempt + 1)))
        } else {
          throw err
        }
      }
    }
  } catch (error) {
    callback(error)
  }
}`

if (cjsContent.includes(cjsBroken)) {
  cjsContent = cjsContent.replace(cjsBroken, cjsFixed)
  writeFileSync(loaderCjsPath, cjsContent)
  console.log('[nextra-patch] Strategy 4 applied (loader.cjs retry on TDZ)')
} else if (cjsContent.includes('_importPromise')) {
  console.log('[nextra-patch] Strategy 4 already applied')
} else {
  console.log('[nextra-patch] Strategy 4: loader.cjs pattern not found')
  console.log('[nextra-patch] loader.cjs content:', JSON.stringify(cjsContent.slice(0, 300)))
}
