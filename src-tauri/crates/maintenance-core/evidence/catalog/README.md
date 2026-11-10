# Catalog 原生检查恢复

本目录独立 crate 可编译，所有 I/O 均通过调用方显式路径和 `CatalogFs`；不发现真实用户目录、不运行任何原产品。`RouterState` / `Provider` / `LegacyAlias` 是检查所需原生字段的类型化投影，并非声称完整恢复原序列化 schema。`Provider.models` 投影每个 `RelayModelEntry` 的模型标识 String（native +0x10），其余字段不参与本检查。

## 观察证据

证据主根为 `../../pseudocode/`；`evidence/*.asm` 由本地 llvm-objdump 对只读 `AiMaMi.arm64` 直接导出。主函数的控制流来自已有 Ghidra C，并逐项核对调用、常量和分支；helper 直接读 ARM64，未把反汇编当作运行时观察。

| 已恢复内容 | 直接证据 |
| --- | --- |
| 路由禁用、config不可读跳过；真实账号有静态目录即error；无账号缺路径warning | `decomp-check_catalog_path_validity.c:36-51,338-398`；`evidence/path.asm` @1003898bc-100389a94 |
| 路径落在管理根才metadata；任何metadata失败均判文件不存在；路径原值传metadata | 同C:125-198；`evidence/path.asm` @100389a94 后分支 |
| 外部路径大小写敏感 contains：cc-switch/cc_switch/ccswitch/codex-switch | 同C:200-286；`evidence/path.asm` @100389dxx-100389eb8；原始常量 @10130e738 |
| 外部普通路径及第三方残留各自标题 | `evidence/external_title.asm` @1003eca74，常量 @10130e744 长42；`evidence/path.asm:399-416`，常量 @10130e76e 长52 |
| normalize仅反斜杠替换、折叠重复斜杠、移除所有尾斜杠；不解析..或symlink | `evidence/normalize.asm` @100388f50-10038923c；`evidence/under_dir.asm` @1003724b8-10037265c |
| 固定 catalog 文件检查：metadata、UTF8读取、空白、JSON解析、models数组 | `decomp-check_catalog_integrity.c:85-441`；`evidence/integrity.asm` @10037795c-100377ef4 |
| 仅取models各元素的string slug，忽略无效字段，保留空字符串，去重 | `evidence/slug_fold.asm:40-53` @1008079ec-100807a18，key @1013b0af2 为slug |
| 期望项按active IDs找第一个同ID provider，找不到跳过，不额外查enabled | `evidence/active_fold.asm` @1008042d8-10080438c（provider stride 0x100） |
| exposed_models非空直接复制，否则用provider.model生成单条 | `evidence/exposed_models.asm:14-50` @1007992f4-100799370 |
| active slug为provider.id::entry.model | `evidence/model_into_fold.asm:28-45` @10050d59c-10050d5e8，format bytes @10151cfd8 为 `c0 02 3a 3a c0 00` |
| 无账号追加legacy alias的可选字段(+0x30)，canonical严格白名单 | `evidence/no_account.asm` @10036fa2c-10036fa64；`evidence/alias_fold.asm:46-54` @100804088-10080409c；`evidence/canonical_alias.asm` @100370184-100370390；静态字符串 @10130c594 长50 |
| missing双模式都检查；extra只无账号检查；两者排序后warning | `decomp-check_catalog_integrity.c:442-619`；`evidence/integrity.asm` @100378028-100378324 |
| top-level reader先按多行字符串完整性组合逻辑行，忽略6条marker行但保留其内容；见首section后不再取顶层项 | `evidence/complete_lines.asm` @10078dbe4-10078dd54；`evidence/top_lines.asm` @10078c01c-10078c528 |
| assignment key仅ASCII字母数字和-_；reader第一同名项即返回；值按单项TOML解析 | `evidence/assignment_key.asm` @10078ae60-10078b0c8；`evidence/read_top.asm` @10078c79c-10078c9fc；`evidence/extract_value.asm` @10078b0cc-10078b410 |

`evidence/read_constants.py` 根据 Mach-O segment VA 映射读取字符串并解码C中的字面分配；该脚本输出中零字节项表示脚本未支持该写入形式，并非实际产品的文案。实现所用中文标题另通过直接常量读取核对。

## 验证

- `cargo test --offline --manifest-path codex-direct/catalog/Cargo.toml`：11 tests通过。
- 包括真正临时文件的无效UTF8/目录读失败；合成Fs覆盖metadata NotFound/PermissionDenied、read错误、空白、坏JSON、非数组models。
- 集合测试覆盖去重、无效条目、空slug、排序、native允许extra、no-account拒extra、provider fallback和first match、5个兼容slot精确匹配。
- reader/path测试覆盖多行、转义、引号key拒绝、坏类型第一项、marker内容、大小写、根目录前缀边界和lexical ..行为。

## 目录构造恢复

新增 `src/builder.rs`，保留两check既有接口。`make_relay_model_info` 实际生成所有原生插入字段；`prepare_router_catalog` 从类型化provider/model/alias输入构造整份目录，包含无账号隐藏兼容行，不要求调用方提供预生成JSON。`encode_prepared_catalog` 使用pretty JSON。具体字段类型、条件缺省和原C行见 `SCHEMA.md`。

新增证据：`../decompiled/{build_relay_models,catalog_reasoning_levels,anthropic_model_profile,normalize_model_name,brand_for_model,rejects_hosted_web_search,validate_legacy_no_account_aliases,validate_relay_provider_id,is_no_account_slot_slug,model_matches_family}.c`。输入二进制SHA256均由导出文件头记录为 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。

- `catalog_reasoning_levels` @10036d090：不是array[string]，而是effort/description对象数组；native Responses默认medium+low/medium/high/xhigh；特殊DeepSeek-v4-flash为high+low/high/max；普通relay按brand生成单项或DeepSeek两项；Anthropic按profile及日期/版本边界选择。
- `model_matches_family` @1002007bc：两侧边界仅`- / : @`，数字版本后缀不误判父族，8位日期后缀允许。`evidence/family.asm`直接核对；`evidence/effort_fold.asm` @100805608 映射Anthropic effort描述。
- brand构造枚举实际为Auto/DeepSeek/Kimi/MiniMax/MiMo/GLM/OpenRouter。托管搜索拒绝规则检查Responses wire、MiMo/MiniMax brand、mimo/minimax/longcat/qwen3-coder名称和4个URL子串。URL表 @101817460，Anthropic effort表 @101817318..1018173c8。
- `prepare_router_catalog.c` @10036bed0：无条件验证aliases；按active ID找首provider、保留模型顺序及重复项；原prepare调用build时templates恒空，故其全部可达内容构造分支现已实现。`evidence/alias_find.asm` @10050f9fc确认alias按原始target_model精确匹配，缺失则用临时provider fallback模型。
- 新增7项构造分支测试，累计19 tests通过；覆盖固定字段、JSON类型、native/DeepSeek/MiMo/MiniMax/Anthropic能力、template优先级、context override、family版本/日期边界、别名验证和fallback、重复及顺序、pretty编码回读。

字段及prepare内容构造已恢复。`build_relay_models_with_templates` 也恢复非空模板：原始slug精确匹配优先；GPT-5.6变体否则借最长规范化slug前缀模板的推理字段，等长取最后候选；非精确模板不复制base_instructions/model_messages。`evidence/template_prefix_fold.asm` @1008029a4-100802d78证实这是最长前缀选择，**没有推理档位交集计算**。新增测试覆盖最长前缀、同长末项、只借推理字段。

后续状态、CLI signature、原子发布/acknowledge及原生Provider/Model缺字段规则见[state-publication.md](state-publication.md)；credential schema、实际存储、旧ID/明文key迁移、revision提交和失败补偿见[credentials-native.md](credentials-native.md)。这些已经实现，不再属于未恢复项。原binary可见toml_edit0.20.2，本地恢复crate使用缓存toml0.9.12，罕见语法跨版本等价与原binary动态差分尚未验证。所有测试均为合成数据及恢复实现的分支测试，不冒充原产品动态对照。

Atelier强类型gateway接入、原始model ID适配及revision CAS文件锁证据见[app-adapter.md](app-adapter.md)。
