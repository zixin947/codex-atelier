<script setup lang="ts">
defineProps<{ name: string; size?: number }>()
const paths: Record<string, string> = {
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
