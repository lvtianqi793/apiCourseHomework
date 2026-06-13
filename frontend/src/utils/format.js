export function formatCount(n) {
  if (n >= 10000) return parseFloat((n / 10000).toFixed(1)) + '万'
  if (n >= 1000) return parseFloat((n / 1000).toFixed(1)) + '千'
  return n
}
