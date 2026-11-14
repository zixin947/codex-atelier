package agent

import (
	"bytes"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"

	"github.com/pelletier/go-toml/v2"
	"github.com/zixin947/codex-atelier/router/internal/edit"
	"github.com/zixin947/codex-atelier/router/internal/sessions"
)

// Called only by an explicit model-setting action, never by the read-only listing.
func migrateCodexIdentity(home string) error {
	config := filepath.Join(home, "config.toml")
	before, err := os.ReadFile(config)
	if err != nil {
		return err
	}
	var doc map[string]any
	if err := toml.Unmarshal(before, &doc); err != nil {
		return err
	}
	providers, _ := doc["model_providers"].(map[string]any)
	legacy, _ := providers["magpie"].(map[string]any)
	if legacy == nil {
		return errors.New("旧 Provider 定义缺失，请先修复配置")
	}
	canonical := make(map[string]any, len(legacy))
	for key, value := range legacy {
		canonical[key] = value
	}
	canonical["name"] = "Atelier"
	existing, exists := providers["atelier"]
	if exists && !reflect.DeepEqual(existing, canonical) {
		return errors.New("atelier Provider 已被其他配置使用，未执行迁移")
	}
	if sessions.CodexThreadsBusyOn(home, "magpie") {
		return errors.New("旧 Provider 仍有运行中的线程，请结束任务后再迁移")
	}
	staged, err := os.CreateTemp(home, ".atelier-identity-*.toml")
	if err != nil {
		return err
	}
	name := staged.Name()
	defer os.Remove(name)
	candidate := append([]byte(nil), before...)
	if !exists {
		block, err := toml.Marshal(map[string]any{"model_providers": map[string]any{"atelier": canonical}})
		if err != nil {
			staged.Close()
			return err
		}
		block = bytes.Replace(block, []byte("[model_providers]\n"), nil, 1)
		candidate = append(candidate, '\n')
		candidate = append(candidate, block...)
	}
	_, err = staged.Write(candidate)
	closeErr := staged.Close()
	if err != nil {
		return err
	}
	if closeErr != nil {
		return closeErr
	}
	if doc["model_provider"] == "magpie" {
		if err := edit.SetTOMLTop(name, edit.KV{Path: "model_provider", Value: "atelier"}); err != nil {
			return err
		}
	}
	tables, err := edit.TOMLTables(name)
	if err != nil {
		return err
	}
	for _, table := range tables {
		if !strings.HasPrefix(table, "profiles.") {
			continue
		}
		values, err := edit.GetTOMLTable(name, table)
		if err != nil {
			return err
		}
		if values["model_provider"] == "magpie" {
			if err := edit.SetTOMLKey(name, table, "model_provider", "atelier"); err != nil {
				return err
			}
		}
	}
	candidate, err = os.ReadFile(name)
	if err != nil {
		return err
	}
	backup, err := os.CreateTemp(home, "config-before-atelier-*.toml")
	if err != nil {
		return err
	}
	_, err = backup.Write(before)
	closeErr = backup.Close()
	if err != nil {
		return err
	}
	if closeErr != nil {
		return closeErr
	}
	paged, err := sessions.RetagPagedCodexThreads(home, "magpie", "atelier")
	if err != nil {
		_, undoErr := sessions.RetagPagedCodexThreadIDs(home, "atelier", "magpie", paged)
		return errors.Join(err, undoErr)
	}
	moved, err := sessions.MoveCodexThreads(home, "magpie", "atelier")
	ids := make([]string, 0, len(moved))
	for _, move := range moved {
		ids = append(ids, move.ID)
	}
	if err == nil && slices.Contains(sessions.CodexThreadProviders(home), "magpie") {
		err = errors.New("仍有旧 Provider 线程未能迁移，请结束任务并检查记录文件后重试")
	}
	if err == nil {
		current, readErr := os.ReadFile(config)
		if readErr != nil {
			err = readErr
		} else if !bytes.Equal(current, before) {
			err = errors.New("迁移期间配置被外部修改，请重试")
		}
	}
	if err == nil {
		err = edit.WriteAtomic(config, candidate)
	}
	if err != nil {
		_, filesErr := sessions.MoveCodexThreadIDs(home, "atelier", "magpie", ids)
		_, pagedErr := sessions.RetagPagedCodexThreadIDs(home, "atelier", "magpie", paged)
		return errors.Join(fmt.Errorf("Atelier 身份迁移失败：%w", err), filesErr, pagedErr)
	}
	return nil
}
