// @flow
const urlRegexp = /^https?:\/\//i

export default (url: string): bool => urlRegexp.test(url)
