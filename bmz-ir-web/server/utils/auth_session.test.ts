import { afterAll, expect, mock, test } from 'bun:test'
import { createClient } from '@libsql/client'
import { drizzle } from 'drizzle-orm/libsql'
import { readdir, readFile } from 'node:fs/promises'
import type { H3Event } from 'h3'
import * as schema from '../db/schema'

const client = createClient({ url: 'file::memory:' })
const db = drizzle(client, { schema })
mock.module('hub:db', () => ({ db, schema }))
const {
  createAuthTokens,
  findUserByWebSession,
  revokeUserSessions,
  revokeWebSession,
  listUserSessions,
  revokeUserSessionById,
} = await import('./auth_tokens')
const { resolveIrUser } = await import('./auth')
const migrations = new URL('../../../server/db/migrations/sqlite/', import.meta.url)
for (const name of (await readdir(migrations)).filter((name) => name.endsWith('.sql')).sort()) {
  await client.executeMultiple(await readFile(new URL(name, migrations), 'utf8'))
}
await db
  .insert(schema.users)
  .values({ id: 'web-player', email: 'web@example.invalid', passwordHash: '' })
afterAll(() => client.close())

test('cookies respect password revocation, individual logout, expiry and account boundaries', async () => {
  const now = Date.now()
  const first = await createAuthTokens('web-player', { now })
  const second = await createAuthTokens('web-player', { now })
  const session = { user: { id: 'web-player' }, secure: { sessionGroupId: first.sessionGroupId } }
  Object.assign(globalThis, { getUserSession: async () => session })
  const event = { node: { req: { headers: {} } } } as H3Event
  expect(await resolveIrUser(event)).toMatchObject({ id: 'web-player' })
  expect(await findUserByWebSession('another-player', first.sessionGroupId, now)).toBeNull()
  expect(
    await findUserByWebSession('web-player', first.sessionGroupId, now + 31 * 86400_000),
  ).toBeNull()
  await revokeWebSession('web-player', first.sessionGroupId)
  expect(await resolveIrUser(event)).toBeNull()
  session.secure.sessionGroupId = second.sessionGroupId
  expect(await resolveIrUser(event)).not.toBeNull()
  await revokeUserSessions('web-player', 'password_changed')
  expect(await resolveIrUser(event)).toBeNull()
  await createAuthTokens('web-player')
  const active = await listUserSessions('web-player')
  expect(active).toHaveLength(1)
  expect(await revokeUserSessionById('web-player', active[0]!.id)).toBeTrue()
  expect(await listUserSessions('web-player')).toHaveLength(0)
  Object.assign(globalThis, { getUserSession: async () => ({ user: { id: 'web-player' } }) })
  expect(await resolveIrUser(event)).toBeNull()
})
