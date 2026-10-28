<script setup lang="ts">
defineProps<{ name: string; size?: number }>()
const paths: Record<string, string> = {
  palette:
    'M12 3a9 9 0 1 0 0 18h2a2 2 0 0 0 0-4h-1a2 2 0 0 1 0-4h4a4 4 0 0 0 4-4c0-4-5-6-9-6z M7 8h1 M11 6h1 M16 7h1 M5 12h1',
  ticket: 'M3 5h18v5a2 2 0 0 0 0 4v5H3v-5a2 2 0 0 0 0-4z M15 5v3 M15 11v2 M15 16v3',
  pencil: 'm4 16 12-12 4 4-12 12H4z M13 7l4 4',
  copy: 'M8 8h13v13H8z M16 8V3H3v13h5',
  gem: 'M3 8 7 3h10l4 5-9 13z M3 8h18 M7 3l5 18 5-18',
  subscription: 'M12 3 4 7v7l8 7 8-7V7z M8 12l3 3 5-6',
  calendar: 'M4 5h16v16H4z M8 2v6 M16 2v6 M4 10h16 M8 14h2 M14 14h2 M8 17h2',
  plus: 'M12 4v16 M4 12h16',
  list: 'M8 5h13 M8 12h13 M8 19h13 M3 5h1 M3 12h1 M3 19h1',
  play: 'M7 3v18l14-9z',
  trash: 'M4 6h16 M9 6V3h6v3 M6 6v15h12V6 M10 10v7 M14 10v7',
  download: 'M12 3v12 M7 10l5 5 5-5 M3 16v5h18v-5',
  upload: 'M12 16V4 M7 9l5-5 5 5 M3 16v5h18v-5',
  grid: 'M3 3h7v7H3z M14 3h7v7h-7z M3 14h7v7H3z M14 14h7v7h-7z',
  chip: 'M7 7h10v10H7z M9 1v4 M15 1v4 M9 19v4 M15 19v4 M1 9h4 M1 15h4 M19 9h4 M19 15h4',
  box: 'm12 2 9 5v10l-9 5-9-5V7z M3 7l9 5 9-5 M12 12v10',
  pulse: 'M2 12h5l3-8 4 16 3-8h5',
  user: 'M16 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0 M4 22v-3a8 8 0 0 1 16 0v3',
  history: 'M3 4v6h6 M3 10a9 9 0 1 1 1 8 M12 7v5l3 2',
  heart: 'M12 21 3 12C-3 4 7-1 12 6c5-7 15-2 9 6z',
  settings: 'M9 3h6l1 4 4 1v8l-4 1-1 4H9l-1-4-4-1V8l4-1z M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0',
  arrow: 'M4 12h15 M14 7l5 5-5 5',
  search: 'M16 10a6 6 0 1 1-12 0 6 6 0 0 1 12 0 M15 15l6 6',
  check: 'M4 12l5 5L20 6',
  alert: 'm12 3 10 18H2z M12 9v5 M12 17v1',
  close: 'M5 5l14 14 M19 5 5 19',
  refresh: 'M20 4v6h-6 M20 10a8 8 0 1 0 0 6',
  shield: 'M12 2 3 6v6c0 5 9 10 9 10s9-5 9-10V6z M8 12l3 3 5-6',
  terminal: 'm5 7 5 5-5 5 M13 17h6',
  chevron: 'm9 5 7 7-7 7',
  book: 'M3 3h7l2 2 2-2h7v17h-7l-2 2-2-2H3z M12 5v17',
  plug: 'M8 2v6 M16 2v6 M5 8h14v4a7 7 0 0 1-14 0z M12 19v4',
}
</script>
<template>
  <svg
    :width="size || 18"
    :height="size || 18"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.65"
    stroke-linecap="square"
    stroke-linejoin="miter"
    aria-hidden="true"
  >
    <path :d="paths[name] || paths.box" />
  </svg>
</template>
