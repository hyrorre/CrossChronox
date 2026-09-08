declare module '#auth-utils' {
  interface SecureSessionData {
    sessionGroupId?: string
  }
  interface User {
    id: string
    email: string
    displayName: string
  }
}
