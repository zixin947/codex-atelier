# Endfield contour runtime

Source: https://github.com/ymh0000123/dsh-theme-endfield

Pinned commit: `359bfad889b4f3d2e79515c4145f624f34ba84f1`.

- `endfield-contour.js`: upstream `src/client/20-contour.js` inside an Atelier module/lifecycle wrapper. The only replacement inside the original fragment is `findAppFrame`, which receives the explicit Vue workstation element instead of locating DSH-specific classes.
- `contour-worker-source.js`: unchanged generated worker fragment from `src/client/21-contour-worker.embed.js`, with module export and attribution header added. Its embedded WebGL source retains the upstream higekibaka contribution credit.
- The default renderer remains upstream Canvas2D, with 24 FPS and 2× phase speed. This also preserves the main-thread mouse deformation path. No worker execution or CSP relaxation is enabled by Atelier.
- The original field generation, coverage validation, marching squares, spline smoothing, pointer history, line colors, density, frame throttling, reduced-motion behavior and teardown remain in the vendored fragment.
- The wrapper supplies local background options, attaches scroll hooks to the explicit host, and disconnects observers/listeners on disposal. It does not import DSH account, authentication, shell, audio or network code.

Preserve upstream comments/formatting when updating. Full MIT license and source attribution: [THIRD_PARTY_NOTICES.md](../../../../THIRD_PARTY_NOTICES.md).
