const urlRegexp = /^https?:\/\//i

export default url => urlRegexp.test(url)
