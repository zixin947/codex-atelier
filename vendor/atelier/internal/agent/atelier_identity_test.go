package agent

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/zixin947/codex-atelier/router/internal/edit"
)

func TestAtelierIdentityMigration(t *testing.T) {
	const original = `# user configuration
model_provider = "magpie"
model = "fake/m1"
[model_providers.magpie]
name = "magpie"
base_url = "http://127.0.0.1:19999/v1"
wire_api = "responses"
[model_providers.custom]
name = "keep me"
base_url = "https://example.invalid/v1"
[profiles.work]
model_provider = "magpie"
model = "fake/m1"
`
	home, read := codexHome(t, "", original)
	dir := filepath.Join(home, ".codex")
	if err := migrateCodexIdentity(dir); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(dir, "config.toml")
	for _, table := range []string{"model_providers.magpie", "model_providers.atelier", "model_providers.custom"} {
		values, err := edit.GetTOMLTable(path, table)
		if err != nil || len(values) == 0 {
			t.Fatalf("missing %s: %v", table, err)
		}
	}
	if value, _ := edit.GetTOMLTop(path, "model_provider"); value != "atelier" {
		t.Fatal(value)
	}
	profile, _ := edit.GetTOMLTable(path, "profiles.work")
	if profile["model_provider"] != "atelier" || profile["model"] != "fake/m1" {
		t.Fatal(profile)
	}
	if !strings.Contains(read(), "# user configuration") {
		t.Fatal("lost comments")
	}
	backups, _ := filepath.Glob(filepath.Join(dir, "config-before-atelier-*.toml"))
	if len(backups) != 1 {
		t.Fatal(backups)
	}
	saved, err := os.ReadFile(backups[0])
	if err != nil || string(saved) != original {
		t.Fatal("backup differs", err)
	}
	if err := migrateCodexIdentity(dir); err != nil {
		t.Fatal("repeat migration", err)
	}
}

func TestAtelierIdentityPreservesConflictingProvider(t *testing.T) {
	const original = "model_provider = \"magpie\"\n[model_providers.magpie]\nname = \"magpie\"\n[model_providers.atelier]\nname = \"another provider\"\n"
	home, read := codexHome(t, "", original)
	if err := migrateCodexIdentity(filepath.Join(home, ".codex")); err == nil {
		t.Fatal("accepted conflicting identity")
	}
	if read() != original {
		t.Fatal("changed config on conflict")
	}
}

func TestAtelierIdentityRollsBackIncompleteThreadMigration(t *testing.T) {
	const original = "model_provider = \"magpie\"\n[model_providers.magpie]\nname = \"magpie\"\n"
	home, read := codexHome(t, "", original)
	dir := filepath.Join(home, ".codex")
	codexThreads(t, dir, "magpie") // A real supported index row whose rollout is unavailable.
	if err := migrateCodexIdentity(dir); err == nil {
		t.Fatal("activated with an unmigrated thread")
	}
	if read() != original {
		t.Fatal("changed active provider on incomplete migration")
	}
}
