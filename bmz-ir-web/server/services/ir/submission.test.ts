import { afterAll, beforeEach, describe, expect, mock, spyOn, test } from 'bun:test'
import { createClient } from '@libsql/client'
import { drizzle } from 'drizzle-orm/libsql'
import { readdir, readFile } from 'node:fs/promises'
import * as schema from '../../db/schema'
import type { IrScoreSubmission } from '../../../shared/types/ir'

// Isolated in-memory SQL database: exercise the real service and atomic score/best batch.
const client = createClient({ url: 'file::memory:' })
const db = drizzle(client, { schema })
mock.module('hub:db', () => ({ db, schema }))
const { submitScore } = await import('./submission')
const { IrIdempotencyCollisionError } = await import('./idempotency')
const migrations = new URL('../../../../server/db/migrations/sqlite/', import.meta.url)
for (const name of (await readdir(migrations)).filter((name) => name.endsWith('.sql')).sort()) {
  await client.executeMultiple(await readFile(new URL(name, migrations), 'utf8'))
}
await db
  .insert(schema.users)
  .values({ id: 'player', email: 'test@example.invalid', passwordHash: '' })
await db.insert(schema.profiles).values({ id: 'player' })
const user = { id: 'player', displayName: 'Player' }

afterAll(() => client.close())
beforeEach(async () => {
  await db.delete(schema.bestScores)
  await db.delete(schema.scores)
  await db.delete(schema.charts)
})

function submission(): IrScoreSubmission {
  const counts = { pgreat: 0, great: 0, good: 0, bad: 0, poor: 0, empty_poor: 0 }
  return {
    client: { name: 'bmz', version: '0.3.0', platform: 'windows' },
    chart: { sha256: 'a'.repeat(64) },
    rule: {
      play_mode: 'normal',
      key_mode: '7k',
      gauge: 'normal',
      ln_policy: 'ForceLn',
      effective_ln_mode: 'ln',
      rule_mode: 'Beatoraja',
      judge_algorithm: 'combo',
      scoring: 'bms_ex_score_v1',
    },
    result: {
      clear: 'clear',
      played_at: 1234567890,
      judges: { fast: { ...counts, pgreat: 100 }, slow: counts },
      ex_score: 200,
      max_combo: 100,
      notes: 100,
      min_bp: 0,
      min_cb: 0,
    },
    play_options: { device_type: 'keyboard' },
    idempotency_key: 'bmz-score-95',
  }
}

describe('score submission idempotency', () => {
  test.each([false, true])(
    'checks the concurrent insert conflict path (collision=%s)',
    async (collision) => {
      const payload = submission()
      payload.play_options.seed = 123
      const first = await submitScore(user, payload, [], 10)
      const stored = await db.select().from(schema.scores)
      const best = await db.select().from(schema.bestScores)
      // Hide the first lookup to simulate a competing request committing after it.
      const lookup = spyOn(db.query.scores, 'findFirst').mockResolvedValueOnce(undefined)
      const log = spyOn(console, 'warn').mockImplementation(() => {})
      try {
        if (collision) {
          payload.result.ex_score = 201
          await expect(submitScore(user, payload, [], 10)).rejects.toBeInstanceOf(
            IrIdempotencyCollisionError,
          )
        } else {
          expect(await submitScore(user, payload, [], 10)).toMatchObject({
            accepted: true,
            score_id: first.score_id,
          })
        }
        expect(await db.select().from(schema.scores)).toEqual(stored)
        expect(await db.select().from(schema.bestScores)).toEqual(best)
      } finally {
        lookup.mockRestore()
        log.mockRestore()
      }
    },
  )
  test('identical retries return the existing result without inserting or changing scores', async () => {
    const payload = submission()
    payload.play_options.seed = 123
    const first = await submitScore(user, payload, [], 10)
    const stored = await db.select().from(schema.scores)
    const best = await db.select().from(schema.bestScores)
    const retry = await submitScore(
      user,
      {
        ...payload,
        evidence: { obsolete: true },
        play_options: { ...payload.play_options, seed: '123' },
      },
      [],
      10,
    )
    expect(first.accepted).toBe(true)
    expect(retry).toMatchObject({ accepted: true, score_id: first.score_id, best_updated: false })
    expect(await db.select().from(schema.scores)).toEqual(stored)
    expect(await db.select().from(schema.bestScores)).toEqual(best)
  })

  test.each(['chart', 'ex_score', 'rule_mode', 'ln_policy', 'double_option', 'judges'])(
    'rejects a collision in %s without changing either score or best',
    async (field) => {
      const payload = submission()
      const first = await submitScore(user, payload, [], 10)
      const stored = await db.select().from(schema.scores)
      const best = await db.select().from(schema.bestScores)
      const changed = structuredClone(payload)
      if (field === 'chart') changed.chart.sha256 = 'b'.repeat(64)
      if (field === 'ex_score') changed.result.ex_score = 2776
      if (field === 'rule_mode') changed.rule.rule_mode = 'Dx'
      if (field === 'ln_policy') changed.rule.ln_policy = 'ForceCn'
      if (field === 'double_option') changed.play_options.double_option = 'battle'
      if (field === 'judges') changed.result.judges.fast.great = 1
      const log = spyOn(console, 'warn').mockImplementation(() => {})
      try {
        await expect(submitScore(user, changed, [], 10)).rejects.toBeInstanceOf(
          IrIdempotencyCollisionError,
        )
        expect(log).toHaveBeenCalledWith('IR idempotency key collision', {
          playerId: user.id,
          idempotencyKey: payload.idempotency_key,
          existingScoreId: first.score_id,
          existingChartSha256: payload.chart.sha256,
          incomingChartSha256: changed.chart.sha256,
        })
      } finally {
        log.mockRestore()
      }
      expect(await db.select().from(schema.scores)).toEqual(stored)
      expect(await db.select().from(schema.bestScores)).toEqual(best)
      expect((await db.select().from(schema.charts)).length).toBe(1)
    },
  )
})
