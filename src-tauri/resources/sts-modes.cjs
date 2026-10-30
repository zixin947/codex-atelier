'use strict'

// Executed only by the fixed Rust adapter after verifying the pinned upstream
// source. This module never runs a hook, sends a prompt, or changes Hook trust.
const fs = require('node:fs')
const path = require('node:path')
const crypto = require('node:crypto')
const MODES = new Set(['unconfirmed', 'answer', 'review', 'monitor', 'change', 'open'])
const LEVELS = new Set(['watch', 'guard', 'lock', 'off'])

function failure(code) {
  const error = new Error(code)
  error.code = code
  throw error
}
function regular(file, optional = false) {
  let stat
  try {
    stat = fs.lstatSync(file)
  } catch (error) {
    if (optional && error.code === 'ENOENT') return false
    throw error
  }
  if (
    !stat.isFile() ||
    stat.isSymbolicLink() ||
    stat.size > 512 * 1024 ||
    (process.getuid && stat.uid !== process.getuid())
  )
    failure('UNSAFE_FILE')
  return true
}
function revision(file) {
  regular(file)
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex')
}
function paths(value) {
  if (
    !Array.isArray(value) ||
    value.length === 0 ||
    value.length > 128 ||
    value.some((p) => typeof p !== 'string' || !p.trim() || p.length > 1024 || /[\0\r\n]/.test(p))
  )
    failure('INVALID_PATHS')
  return [...new Set(value.map((p) => p.trim()))]
}
function atomic(file, value) {
  fs.mkdirSync(path.dirname(file), { recursive: true, mode: 0o700 })
  if (fs.lstatSync(path.dirname(file)).isSymbolicLink()) failure('UNSAFE_FILE')
  const temporary = `${file}.${crypto.randomUUID()}.tmp`
  try {
    const fd = fs.openSync(temporary, 'wx', 0o600)
    try {
      fs.writeFileSync(fd, JSON.stringify(value) + '\n')
      fs.fsyncSync(fd)
    } finally {
      fs.closeSync(fd)
    }
    fs.renameSync(temporary, file)
  } finally {
    try {
      fs.unlinkSync(temporary)
    } catch (error) {
      if (error.code !== 'ENOENT') throw error
    }
  }
}
function execute(input) {
  if (
    !input ||
    !['status', 'apply', 'rollback'].includes(input.action) ||
    typeof input.sessionId !== 'string' ||
    !/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i.test(input.sessionId)
  )
    failure('INVALID_REQUEST')
  const { readState, updateSession, statePath, sessionKey } = require(
    path.join(input.sourceRoot, 'src/state.cjs'),
  )
  const { parseContractPrompt } = require(path.join(input.sourceRoot, 'src/contracts.cjs'))
  const file = statePath(input.sessionId, input.dataDir)
  for (const directory of [input.dataDir, path.dirname(file)]) {
    const stat = fs.lstatSync(directory)
    if (
      !stat.isDirectory() ||
      stat.isSymbolicLink() ||
      (process.getuid && stat.uid !== process.getuid())
    )
      failure('UNSAFE_FILE')
  }
  const backupFile = path.join(input.backupDir, `sts-${sessionKey(input.sessionId)}.json`)
  regular(file) // Never fabricate a session that has not actually run STS.
  function snapshot() {
    const before = revision(file)
    const state = readState(input.sessionId, input.dataDir)
    if (state.storageError) failure('DAMAGED_STATE')
    const after = revision(file)
    if (before !== after) failure('STALE_REVISION')
    const contract = state.contract
    if (!MODES.has(contract.mode) || !LEVELS.has(contract.level)) failure('UNSUPPORTED_STATE')
    if (
      contract.allowedPaths !== null &&
      contract.allowedPaths !== undefined &&
      (!Array.isArray(contract.allowedPaths) ||
        contract.allowedPaths.some((value) => typeof value !== 'string'))
    )
      failure('UNSUPPORTED_STATE')
    let canRollback = false
    if (regular(backupFile, true)) {
      try {
        canRollback = JSON.parse(fs.readFileSync(backupFile, 'utf8')).afterRevision === after
      } catch {
        failure('DAMAGED_BACKUP')
      }
    }
    return {
      mode: contract.mode,
      level: contract.level,
      allowedPaths: contract.allowedPaths ?? null,
      revision: after,
      canRollback,
      source: contract.source || 'unknown',
    }
  }
  if (input.action === 'status') return snapshot()
  if (typeof input.expectedRevision !== 'string') failure('INVALID_REQUEST')
  let desired
  if (input.action === 'rollback') {
    regular(backupFile)
    desired = JSON.parse(fs.readFileSync(backupFile, 'utf8'))
    if (desired.sessionId !== input.sessionId || desired.afterRevision !== input.expectedRevision)
      failure('STALE_REVISION')
    if (
      !desired.contract ||
      !MODES.has(desired.contract.mode) ||
      !LEVELS.has(desired.contract.level)
    )
      failure('DAMAGED_BACKUP')
  } else {
    if (!MODES.has(input.mode) || !LEVELS.has(input.level)) failure('INVALID_REQUEST')
    if (['guard', 'lock'].includes(input.level) && input.mode === 'unconfirmed')
      failure('MODE_REQUIRED')
    if (input.level === 'lock') paths(input.allowedPaths)
  }
  let plannedRevision
  updateSession(input.sessionId, input.dataDir, (state) => {
    if (state.storageError) failure('DAMAGED_STATE')
    if (revision(file) !== input.expectedRevision) failure('STALE_REVISION')
    const previous = structuredClone(state.contract)
    if (desired) {
      state.contract = structuredClone(desired.contract)
    } else {
      // Use the upstream parser for both task and control modes, under its own
      // writer lock. No task text or shell string comes from the frontend.
      if (input.mode === 'unconfirmed' && state.contract.mode !== 'unconfirmed')
        failure('MODE_REQUIRED')
      const mode = input.mode === 'unconfirmed' ? '' : input.mode
      const parsed = parseContractPrompt(`$stop-that-shit ${mode} ${input.level}`, state.contract)
      if (
        parsed.error ||
        parsed.contract.mode !== input.mode ||
        parsed.contract.level !== input.level
      )
        failure('UNSUPPORTED_STATE')
      state.contract = parsed.contract
      if (input.level === 'lock') state.contract.allowedPaths = paths(input.allowedPaths)
      // All other fields (including existing file scope) are deliberately kept.
    }
    state.directiveError = null
    state.directiveWarning = null
    // Writing handlePrompt's lastPromptContext here would suppress the next
    // real UserPromptSubmit injection. Force that next hook to explain the mode.
    state.lastPromptContext = null
    // Upstream writeState serializes this same object with two-space JSON and a
    // final newline. Record the previous contract BEFORE the upstream commits,
    // while still holding its writer lock. A backup failure must abort the write.
    plannedRevision = crypto
      .createHash('sha256')
      .update(JSON.stringify(state, null, 2) + '\n')
      .digest('hex')
    atomic(backupFile, {
      sessionId: input.sessionId,
      contract: previous,
      afterRevision: plannedRevision,
    })
    return { state }
  })
  const saved = revision(file)
  if (saved !== plannedRevision) failure('READBACK_MISMATCH')
  const result = snapshot()
  if (!desired && (result.mode !== input.mode || result.level !== input.level))
    failure('READBACK_MISMATCH')
  return {
    ...result,
    receipt:
      '已保存并读回该对话契约；下一次消息重新注入说明，后续 Hook 读取新模式。实际 Desktop 效果待你验收。',
  }
}

module.exports = { execute }
if (!module.parent) {
  let raw = ''
  process.stdin.setEncoding('utf8')
  process.stdin.on('data', (chunk) => {
    raw += chunk
    if (raw.length > 64 * 1024) {
      process.stderr.write('mode request too large\n')
      process.exit(1)
    }
  })
  process.stdin.on('end', () => {
    try {
      process.stdout.write(JSON.stringify({ ok: true, data: execute(JSON.parse(raw)) }))
    } catch (error) {
      const known = new Set([
        'UNSAFE_FILE',
        'INVALID_PATHS',
        'INVALID_REQUEST',
        'STALE_REVISION',
        'DAMAGED_STATE',
        'UNSUPPORTED_STATE',
        'DAMAGED_BACKUP',
        'MODE_REQUIRED',
        'READBACK_MISMATCH',
        'STS_LOCK_TIMEOUT',
        'STS_LOCK_DAMAGED',
        'ENOENT',
      ])
      process.stdout.write(
        JSON.stringify({ ok: false, code: known.has(error.code) ? error.code : 'ADAPTER_FAILED' }),
      )
    }
  })
}
