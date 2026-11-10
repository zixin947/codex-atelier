# make_relay_model_info：逐赋值 schema

以 `../../pseudocode/decomp-make_relay_model_info.c` 为准。字段来自实际Map插入的key/value，而非字符串池相邻字段名。JSON Value ARM64 tag：0=null，1=bool，2=number，3=string，4=array，5=object；已通过既有check的string/array/object分支交叉确认。

| 字段 | 实际JSON类型及默认/规则 | 原C行 |
| --- | --- | --- |
| slug | string，provider.id + "::" + entry.model | 134-139,205-213 |
| display_name | string；entry.display_name存在且trim非空时使用trim后文本；否则format provider.name与entry.model；entry第二String trim非空另加后缀 | 140-186,246-254 |
| description | string，format provider.name与entry.model | 187-197,289-297 |
| default_reasoning_level | string或null；Anthropic不支持reasoning时null，其余由catalog_reasoning_levels返回 | 489-518 |
| supported_reasoning_levels | array[object{effort:string,description:string}]；有效template数组可原样覆盖，故template输入时元素类型不强制 | 493-496,553-564 |
| shell_type | string固定"shell_command" | 599-616 |
| visibility | string固定"list" | 651-667 |
| supported_in_api | bool固定true | 702-708 |
| priority | number固定100 | 743-750 |
| additional_speed_tiers | array固定[] | 785-795 |
| availability_nux | null | 830-840 |
| upgrade | null | 875-880 |
| base_instructions | string，默认空，部分模型提供专用文本，匹配template可覆盖非空文本 | 915-943,2353-2386 |
| supports_reasoning_summaries | bool，由wire_api、Anthropic profile决定 | 978-989 |
| default_reasoning_summary | string固定"none" | 1024-1042 |
| support_verbosity | bool，native或特殊DeepSeek分支 | 1077-1084 |
| default_verbosity | 条件出现，string固定"low" | 1113-1138 |
| apply_patch_tool_type | 条件出现，string固定"freeform" | 1168-1202 |
| truncation_policy | object，{mode:string("tokens"或"bytes"),limit:number(10000)} | 1238-1345 |
| supports_parallel_tool_calls | bool | 1380-1388 |
| supports_image_detail_original | bool | 1423-1431 |
| supports_search_tool | bool | 1467-1478 |
| web_search_tool_type | 条件出现，string "text"或"text_and_image"；Anthropic最终覆盖成后者 | 1507-1600 |
| context_window | number；entry正整数override优先，否则模型特例、有效template、品牌fallback | 1630-1816 |
| max_context_window | number；entry正整数override优先，否则模型特例、有效template、品牌fallback | 1845-1860 |
| effective_context_window_percent | number；允许template override时取正整数，否则95 | 1889-1941 |
| experimental_supported_tools | array固定[] | 1976-1987 |
| use_responses_lite | bool固定false | 2022-2029 |
| prefer_websockets | 条件出现，bool固定false | 2058-2072 |
| minimal_client_version | 条件出现，string（特例DeepSeek） | 2101-2125 |
| service_tiers | array固定[] | 2161-2170 |
| input_modalities | array[string]，固定text，支持image时另加image | 2205-2268 |
| model_messages | 条件出现，仅复制template中非空object，且template slug与entry原始model一致 | 2297-2432 |

## 顶层和兼容别名

`prepare_router_catalog @10036bed0` 先无条件验证legacy aliases，然后构建模型数组。`param6` 为false时追加legacy rows，true时不追加。顶层object只有此函数实际写入的 `models:array[object]` 与 `aimami_no_account:bool(!param6)`，随后to_vec_pretty。构造产物附带CLI signature；本crate构造内容不应自行探测真实CLI路径。

legacy row从provider id选择真实provider，否则构建临时provider fallback；从exposed_models按alias target匹配，否则用fallback首模型；调用同一个make生成后将slug改为alias可选字段（缺失时null），visibility改"hide"，priority改1000。alias共3个String字段：provider_id、target_model、可选slot_slug；target_model trim必须非空，provider_id必须为可移植文件名且非保留Windows名，slot_slug必须命中5项白名单且彼此唯一。

## 排序

make函数中的Map使用BTreeMap，输出object key按字符串排序。`build_relay_models.c:143-170,530-556` 主循环保留active IDs顺序和每个provider exposed_models顺序，不做模型排序或去重。prepare随后按alias输入顺序追加隐藏行。。
