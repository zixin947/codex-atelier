<script setup lang="ts">
import terrain from '../../assets/hud/terrain-map.svg'
</script>

<template>
  <figure class="hud-map" aria-label="静态地形地图占位">
    <img :src="terrain" alt="灰色地形与黄色高亮区域的地图示意" />
    <svg viewBox="0 0 420 420" fill="none" aria-hidden="true">
      <defs>
        <pattern id="hud-map-grid" width="30" height="30" patternUnits="userSpaceOnUse">
          <path d="M30 0H0V30" stroke="#dce0d5" opacity=".12" stroke-width=".5" />
        </pattern>
      </defs>
      <circle cx="210" cy="210" r="208" fill="url(#hud-map-grid)" />
      <path d="M210 0v420M0 210h420" stroke="#e3e5d2" opacity=".15" stroke-dasharray="2 5" />
      <circle cx="210" cy="210" r="206" stroke="#cbd0bd" opacity=".35" />
      <g stroke="#20231f" fill="#f3f16f">
        <circle cx="273" cy="201" r="3" />
        <circle cx="307" cy="266" r="3" />
        <circle cx="191" cy="176" r="3" />
      </g>
      <path d="M20 120h22m-11-11v22M369 283h22m-11-11v22" stroke="#e7eacb" opacity=".6" />
    </svg>
    <figcaption><small>HUD / MAP</small><b>MAP</b><span>静态地图占位</span></figcaption>
    <span class="hud-map-credit">EARTH RELIEF · NOAA / NATURAL EARTH</span>
  </figure>
</template>
