# 旧根目录状态库清理

原生依据 `fix_state_db_relocation @1003789f4`，输入离线 arm64 SHA-256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。附 decompiled C；静态恢复未执行原程序。

原生门控要求活库路径位于 home/sqlite 下，且配置sqlite_home与该目录按Path精确相等。仅活库在某个自定义目录不足以证明根目录库可以删除。根目录state_*.sqlite匹配不要求数字版本；不递归。

每个主库按原生顺序处理原名、-wal、-shm：metadata可读才处理；复制到state-db-backups/{unix_seconds}.{filename}成功后才删除，逐项失败继续；末尾再扫描三类源文件，任何仍在则报告失败。删除主库的计数与sidecar不同。不是只copy/remove一次，也没有把部分失败伪装成全事务回滚。

应用层将待删文件流式SHA256纳入预览，执行前核对整个目标集合和外部变化，逐份备份同步、私有权限、读回后再删除。没有把DB作为32MiB文本读入内存。原生metadata允许源symlink，备份跟随源内容，删除只unlink旧目录项；临时目录实际测试确认外部目标原件保留。

配置sqlite_home优先于CODEX_SQLITE_HOME；相对路径按显式cwd展开，无波浪号猜测。最高state数字候选的选择与“列出根目录旧库”是两个不同算法；完整Manager跨目录活库选择/重新收敛仍不能据该文件宣称完成。

核心测试覆盖缺少配置、不等配置、三文件成功、复制失败保留、删除失败部分结果、严格配置优先级；应用层覆盖真实preview/apply、备份manifest、预览后新增sidecar/客户端运行、外部目录保护、大库流式处理、备份路径失败及symlink。所有路径均为临时fixture。
