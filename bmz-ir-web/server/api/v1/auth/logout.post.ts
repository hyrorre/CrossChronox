import { getBearerToken } from '../../../utils/auth'
import { revokeToken, revokeWebSession } from '../../../utils/auth_tokens'

interface LogoutBody {
  refresh_token?: string
}

export default defineEventHandler(async (event) => {
  const body = ((await readBody(event).catch(() => undefined)) ?? {}) as LogoutBody
  const accessToken = getBearerToken(event)
  if (accessToken) {
    await revokeToken(accessToken, 'access')
  }
  if (body.refresh_token) {
    await revokeToken(body.refresh_token, 'refresh')
  }

  const session = await getUserSession(event)
  if (session.user?.id && session.secure?.sessionGroupId) {
    await revokeWebSession(session.user.id, session.secure.sessionGroupId)
  }
  await clearUserSession(event)

  return { logged_out: true }
})
