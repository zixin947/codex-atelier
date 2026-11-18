// Inserted at the upstream curve boundary; keeps measured endpoints and packet paths aligned.
if (new URLSearchParams(location.search).get('atelierStyle') === 'endfield') {
  const direction = Math.sign(y3 - y0)
  const radius = Math.min(24, Math.abs(y3 - y0) / 2, Math.abs(mx - x0), Math.abs(x3 - mx))
  return `M${x0} ${y0} L${mx - radius} ${y0} Q${mx} ${y0} ${mx} ${y0 + direction * radius} L${mx} ${y3 - direction * radius} Q${mx} ${y3} ${mx + radius} ${y3} L${x3} ${y3}`
}
