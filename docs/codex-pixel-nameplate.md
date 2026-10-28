# 开屏像素铭牌

用途：替换横版开屏中央铭牌。左侧为 Codex 字样，右侧为用户指定的结形标志像素稿。使用内置图片生成工具制作；不是官方发布的图片素材。

图标轮廓参考 [OpenAI 官方品牌页](https://openai.com/brand/) 的 [Blossom 矢量源图](https://images.ctfassets.net/kftzwdyauwt9/3hUGLn3ypllZ0oa01qOYVq/28e8188e6f11b84c3e876569d492734f/Blossom_Light.svg)。OpenAI / ChatGPT 标志归其权利人所有。未读取或修改真实客户端。

成品：[像素铭牌](../src/assets/branding/codex-pixel-nameplate.png)。清晰参考：[单独标志 SVG](assets/openai-blossom-clean.svg) · [完整官方源 SVG](assets/openai-blossom-reference.svg)。单独版仅保留源图中的干净标志路径、重新设置画布，没有重绘曲线；像素铭牌是据此生成的风格演绎。

## 最终编辑提示词

```text
Use case: precise-object-edit.
Edit target image 1: the existing pixel-art charcoal "Codex" nameplate with a cloud terminal mark on the right.
Reference image 2: the official OpenAI Blossom geometric reference. Use ONLY the clean six-loop interwoven knot mark on the RIGHT HALF of this reference sheet; ignore the left construction diagram, grid, and every other element.
Task: replace ONLY the cloud/terminal logo in the right half of image 1 with the user's requested six-loop OpenAI Blossom knot. Render the knot in the same warm off-white square-pixel style as the Codex lettering, on the same coarse pixel grid, with crisp stair-step edges. Preserve the recognizable six interlocking loops, central hexagonal negative space, and correct over-under ribbon crossings from reference image 2. The symbol must look like the provided knot, not a cloud or a terminal glyph. Keep the knot spacious and legible at a small desktop nameplate size.
Invariants: preserve the EXACT left "Codex" lettering, its wording, placement, scale, pixel shapes, the thin rectangular pixel-cut corner frame, charcoal background, yellow underline under the word, and the two yellow squares near the top-right. Preserve overall canvas dimensions and wide composition. Center the replacement knot in the same right-hand logo area. Use dark negative spaces so all six loops read clearly.
No other text, no cloud, no >_ symbol, no bar chart, no new shapes, no blue gradient, no app tile, no smooth vector treatment, no blur. Opaque background.

```
