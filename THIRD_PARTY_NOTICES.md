# 第三方说明

## dsh-theme-endfield

来源：[ymh0000123/dsh-theme-endfield](https://github.com/ymh0000123/dsh-theme-endfield)。

“终末地 · 野外终端”参考其纸墨/信号色设计与公开令牌，适配为 Atelier 自己的主题。没有引入其宿主运行时或游戏素材。上游许可仅覆盖其原创代码；游戏名称、商标及上游截图等不因该许可获得授权。本项目直接移植了上游 `src/client/20-contour.js` 及其生成的 Worker 源片段，保留原始算法和注释，固定于提交 `359bfad889b4f3d2e79515c4145f624f34ba84f1`。修改包括 Vue 宿主挂载、本地背景选项及生命周期清理。生成源中 higekibaka 的 WebGL 绘制器署名完整保留；默认采用原版 Canvas2D 路径。ENDFIELD 字标样式参照上游 `16-watermark.js` 和主题 CSS；皮肤选择缩略图仍为本项目绘制。

启动开屏另参考同一提交的 `src/client/30-boot-loader.js`：以 TypeScript 重写生命周期与本地初始化衔接，保留 100% 停留、黄色扫屏与淡出节奏。当前横版视觉以用户截图为构图参考，由本项目独立编写 CSS/SVG；圆环与三角铭记自行绘制，没有复制截图里的品牌图像或游戏素材。中央像素铭牌的图案参考单列于下文。没有引入上游的音频、设置服务或宿主通信。

保留上游许可全文：

```text
MIT License

Copyright (c) 2026 ymh0000123

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 像素铭牌的标志参考

`src/assets/branding/codex-pixel-nameplate.png` 由内置图片生成工具制作，右侧按用户指定的结形标志演绎为像素风，并非官方发布的图像。参考来源为 [OpenAI 品牌页](https://openai.com/brand/) 与其 [Blossom_Light.svg](https://images.ctfassets.net/kftzwdyauwt9/3hUGLn3ypllZ0oa01qOYVq/28e8188e6f11b84c3e876569d492734f/Blossom_Light.svg)。原始矢量参考存于 `docs/assets/openai-blossom-reference.svg`，单独标志版本仅调整画布并去掉辅助绘图。OpenAI / ChatGPT 标志的权利归 OpenAI，不因本项目或上游 MIT 许可转让；此图不表示官方合作或背书。完整提示词见 `docs/codex-pixel-nameplate.md`。
