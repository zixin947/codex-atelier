package main

import (
	"bytes"
	"github.com/yetone/magpie/internal/appdir"
	"github.com/yetone/magpie/internal/provider"
	"os"
	"testing"
)

func TestHostPreservesCoreProtocolEntrypoints(t *testing.T) {
	for _, p := range []string{"/v1/chat/completions", "/v1/responses", "/v1/messages", "/v1/messages/count_tokens", "/v1beta/models/gemini:generateContent", "/api/chat", "/api/generate", "/v1/images/generations", "/v1/images/edits", "/v1/audio/speech", "/v1/audio/transcriptions", "/v1/audio/translations", "/v1/videos/job/content", "/v1/embeddings", "/v1/rerank", "/v1/decisions", "/v1/magpie/native/vendor/models", "/mcp/run/example", "/backend-api/codex/responses", "/alpha/search", "/muse-code/models", "/v1/magpie/image/files/asset"} {
		if !protocolPath(p) {
			t.Errorf("core inference path blocked: %s", p)
		}
	}
	for _, p := range []string{"/v1/magpie/keys", "/v1/magpie/keys/oidc", "/v1/magpie/update", "/v1/magpie/restart", "/v1/magpie/quotas/codex-reset", "/v1/magpie/cpu", "/me"} {
		if protocolPath(p) {
			t.Errorf("administration exposed to inference key: %s", p)
		}
	}
}

func TestOriginalUIStartupPreservesNativeProviderAndGroupStore(t *testing.T) {
	root := t.TempDir()
	defer appdir.UseExecutable("")
	c := config{Root: root, Port: 18910, Key: "fixture-local-key-for-native-startup", OriginalUI: true, DiscoverAccounts: true}
	if err := configure(c); err != nil {
		t.Fatal(err)
	}
	original := []byte(`{"providers":[{"id":"saved","name":"saved","chat":"https://example.invalid/v1","key":"fixture"}],"groups":[{"id":"saved-group","members":["saved/model"],"routing":"rotate"}]}`)
	if err := os.WriteFile(provider.Path(), original, 0600); err != nil {
		t.Fatal(err)
	}
	if err := configure(c); err != nil {
		t.Fatal(err)
	}
	actual, err := os.ReadFile(provider.Path())
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(original, actual) {
		t.Fatal("native settings overwritten on restart")
	}
}
