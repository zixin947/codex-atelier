# AiMaMi 1.2.7 配置检查恢复证据

仅静态分析已有 arm64 副本，测试仅用内存合成配置/metadata。没有读取真实 Codex 配置、认证、数据库或操作实际应用。

|实现|证据|结论|
|---|---|---|
|check_config_profile_conflict|pseudocode/decomp-check_config_profile_conflict.c:19-326，入口 0x100390ce8|跳过 managed 块/多行字符串/section；首个非空顶层 profile 报告 error(route on)或warning(off)，fixable=true；缺失输入保留原生 ok/跳过摘要|
|TomlLexicalState|lexical.asm 0x10078da1c-0x10078dbe0|普通状态2，多行基本字符串0，多行字面字符串1；返回的是该行进入时是否普通，而不是结束时状态；普通字符串内反斜杠跳过下一字节，#结束普通行扫描|
|check_config_global_validity|pseudocode/decomp-check_config_global_validity.c:18-325，入口0x10038f3ec|与 profile 使用相同 managed/section/lexical 扫描；逐行 TOML parse，收集实际引用问题；没有本地 provider/profile/推理档位白名单|
|invalid_global_config_reference|global_reference.asm 0x100395fb8-0x100396368|空 model_reasoning_effort；绝对 model_catalog_json 路径不在指定目录内且 metadata 返回 NotFound。其他 metadata 错误不报该问题|
|normalize_path_for_compare|normalize_path.asm 0x100388f50-0x100389264|替换反斜杠为斜杠、重复折叠 //、trim_end_matches('/')。不 canonicalize、不解析 ..|
|path_is_under_dir|path_is_under_dir.asm 0x1003724b8-0x100372660|normalize 后相等或 starts_with(dir+'/')|
|top_level_relay_model_requires_cleanup|model_cleanup.asm 0x10078d874-0x10078da1c|仅 aimami_relay 或 aimami_relay_ 前缀；route off全清，route on仅清活动slug集合不包含的值|
|is_explicit_third_party_marker|third_party_marker.asm 0x100393740-0x100393c04|ASCII小写后匹配 cc-switch、cc_switch、ccswitch、codex-switch、codex_switch、circuit_breaker、provider_router 子串|
|is_cc_switch_fingerprint|fingerprint.asm 0x10037c1ac-0x10037c310|只扫描 structural 非注释行，调用上面的 marker helper|

## 已核对字面量

从 Mach-O __TEXT 直接按VA读取（本副本 __TEXT VA-base 0x100000000，fileoff=0）：
- 0x10130ede3,0x33：顶层 profile 配置（文件不可读，跳过）
- 0x10130ed3b,0x19：无 legacy 顶层 profile
- 0x10152125e：检测到 legacy 顶层 profile = "{}"（Rust压缩fmt，参数位置已按C调用核对）
- profile 的0x8f字节 detail：由 C puVar14 各偏移小端写入逐字重建。
- 0x10130f62c,22：model_reasoning_effort
- 0x10130bacf,18：model_catalog_json
- 0x10130f642,35：model_reasoning_effort 不能为空
- 0x10130f665,49：model_catalog_json 指向不存在的外部路径
- 0x10130eb77,0x2a：全局配置（文件不存在，跳过）
- 0x10130eb59,0x1e：本地配置引用检查通过
- global 的0xa3字节 detail：由 C puVar6 各偏移小端写入逐字重建。

## 边界

原二进制使用的 TOML 依赖精确版本尚未确定，本恢复采用本机缓存 toml 0.9.12；常见配置分支已做合成测试，但未声明所有无效 TOML 文本的跨版本错误恢复完全等价。

原始副本 SHA-256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。

## 当前验证

`cargo fmt`、`cargo test --offline` 已通过，14 个有效分支测试。涵盖 profile 路由开关状态、三组 managed marker、多行字符串伪键、原生 trim 的文本行为；global 空 effort/未知值/类型/语法错误、绝对外部路径 NotFound/权限错误、内部路径边界、问题顺序；cleanup 活动slug保留；fingerprint 的注释/多行与子串行为；第三方URL去除userinfo/query/fragment及无效URL替换@。

`sanitize_third_party_base_url` 对应 decomp-check_config_third_party.c:267-438，URL解析成功与失败是独立分支；原生失败分支不去除尾斜杠，已保留。

四个目标检查的主判断链均已恢复；没有 should_fix/known_status 输入或伪成功实现。未做原生二进制运行差分（本任务禁止执行真实应用），跨TOML/URL依赖版本的异常文本解析差异仍未验证。

## 线程检查共用提取器

`first_section_part` 已从 first_section_part.asm @0x1003735e4-0x100373af8 直接恢复；只识别双引号、保留quoted反斜杠，单引号无特殊语义。`diagnostic_config_provider_ids(text)` 对应 diagnostic_valid_provider_ids @0x10039264c 的配置提取段，由线程分工核对 scanner 边界：raw.trim() 后首[末]，inner不trim，严格前缀model_providers.，余下调用first_section_part；跳过非structural行。尾部注释、[[array]]、inner开头空格均不匹配。当前 `cargo test --offline` 共16测试通过。


## 最终补全：stale 与 third_party

- `src/stale.rs` 恢复 `RouterConfigSurface::parse`（`../decompiled/RouterConfigSurface_parse.c`:6起，0x100362404）和 violations（`../decompiled/config_surface_violations_for_surface.c`:6起，0x100364750）；将顶层provider/profile/model/catalog、legacy localhost URL、真实账号auth标志、精确aimai1 section/endpoint、owned section集合从配置文本计算，不接收状态结论。
- `check_config_stale` 按 `pseudocode/decomp-check_config_stale.c`:54-138 先收集 surface violations，再做 top-level relay model cleanup并避免完全相同说明重复；启用error、关闭warning，没问题才ok。
- 当前模型集合复用 `../catalog/src/lib.rs` 的实际 `RouterState/Provider/LegacyAlias/expected_slugs` 恢复链；本轮又直接反汇编核对 stale 的 Map::fold @0x100804000-0x100804108：alias+0x30 Option值经 canonical_no_account_slot_slug @0x100370184后插入集合，确认与catalog模块同链。
- `normalize_catalog_path` 与普通normalize不同：先trim，再替换反斜杠，最后trim尾斜杠，不collapse重复//。证据normalize_catalog.asm @0x1001f9bc8-0x1001f9e48和 `../decompiled/normalize_catalog_path.c`。
- `router_entry_presence` 仅精确 `[model_providers.aimai1]` section内匹配原始行的 `/codex/router/v1` 子串；注释中的子串亦算，quoted section不匹配。证据 `../decompiled/router_entry_presence.c`:69-176，0x1007881a8。
- `router_provider_requires_openai_auth` 完整TOML解析，只读model_providers.aimai1.requires_openai_auth布尔值；parse失败/缺失/类型错误None，不当作true。证据同名C @0x10078d5c4。
- legacy top-level base_url要求HTTP、host127.0.0.1、非空显式非默认端口、无userinfo/query/fragment、path去尾slash后等于/codex/router/v1。证据 `../decompiled/aimami_router_base_url_port.c`:81-119，0x10078c594。
- `read_top_level_model_line` 在typed reader失败时回退：顶层model前缀加空格或等号，split('=').nth(1)，trim whitespace+doublequote+singlequote+whitespace；不完整section、comment和multiline处理保留原生scanner行为。证据同名C @0x10078b498。
- `find_foreign_providers` 与 `is_known_or_builtin`、`is_explicit_config_owner_residue` 均对照本轮导出的C及局部asm。未知provider不按白名单一概报错：owner明确命名残留或section内存在明确marker才报告；已知集合仍排除aimami_relay namespace；builtin精确aimai1/openai-bundled/openai-primary-runtime，openai忽略ASCII大小写；owner额外字面量是deecodex。证据同名C @0x100376eb0/0x100374940/0x10039b494。
- third_party label优先级：base_url重定向 > fingerprint > section数；组合说明保持section、fingerprint、URL顺序；缺失输入warning且fixable=false。证据 `pseudocode/decomp-check_config_third_party.c`:100-194、201-265、439-516。

最终验证：`cargo fmt`、`cargo test --offline`（32个有效测试）通过，包含四个检查的成功/异常/跳过分支、route on/off、Native/NoAccount实际模型数据、section/marker/quote/multiline边界、权限错误与缺失路径、URL脱敏和原生文本quirks。仅合成配置与metadata，无真实用户数据探测。
