const urlRegexp = /^https?:\/\//i

export default (url: string): boolean => urlRegexp.test(url)
