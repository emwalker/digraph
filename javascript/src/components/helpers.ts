function topicPath(id: string): string {
  return `/topics/${id}`
}

function backgroundColor(hexColor: string) {
  return hexColor === '' ? '' : `${hexColor}33`
}

function borderColor(hexColor: string) {
  return hexColor === '' ? '' : `${hexColor}ff`
}

export { topicPath, backgroundColor, borderColor }
