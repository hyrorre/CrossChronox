import { findUserByWebSession } from '../utils/auth_tokens'

export default defineNitroPlugin(() => {
  sessionHooks.hook('fetch', async (session, event) => {
    if (!session.user) return
    const group = session.secure?.sessionGroupId
    if (!group || !(await findUserByWebSession(session.user.id, group))) {
      delete session.user
      delete session.secure
      await clearUserSession(event)
    }
  })
})
