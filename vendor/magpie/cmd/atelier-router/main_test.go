package main

import (
	"bytes"
	"github.com/yetone/magpie/internal/appdir"
	"github.com/yetone/magpie/internal/gateway"
	"github.com/yetone/magpie/internal/provider"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestInferenceAccessUsesNativeLoopbackCredentials(t *testing.T) {
	t.Setenv("MAGPIE_NO_ACCOUNTS", "1")
	defer appdir.UseExecutable("")
	c := config{Root: t.TempDir(), Port: 18910, Key: "fixture-local-key-for-native-startup", OriginalUI: true}
	if err := configure(c); err != nil {
		t.Fatal(err)
	}
	native := inferenceAccess(c, "127.0.0.1:18910", gateway.New().AtelierHandler())
	c.OriginalUI = false
	legacy := inferenceAccess(c, "127.0.0.1:18910", http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(200) }))
	for _, tc := range []struct {
		name, key, origin, host, path, forwarded string
		handler                                  http.Handler
		want                                     int
	}{
		{name: "atelier placeholder", key: "atelier", handler: native, want: 200},
		{name: "native default", key: gateway.Token, handler: native, want: 200},
		{name: "native agent", key: gateway.TokenFor("claude"), handler: native, want: 200},
		{name: "native keyless loopback", handler: native, want: 200},
		{name: "browser", key: gateway.Token, origin: "null", handler: native, want: 401},
		{name: "foreign host", key: gateway.Token, host: "evil.invalid:18910", handler: native, want: 403},
		{name: "forwarded caller", key: gateway.Token, forwarded: "203.0.113.1", handler: native, want: 403},
		{name: "administration", key: gateway.Token, path: gateway.KeysPath, handler: native, want: 404},
		{name: "legacy rejects native token", key: gateway.Token, handler: legacy, want: 401},
		{name: "legacy private key", key: c.Key, handler: legacy, want: 200},
	} {
		t.Run(tc.name, func(t *testing.T) {
			path := tc.path
			if path == "" {
				path = "/v1/models"
			}
			r := httptest.NewRequest("GET", "http://127.0.0.1:18910"+path, nil)
			r.RemoteAddr = "127.0.0.1:12345"
			if tc.host != "" {
				r.Host = tc.host
			}
			r.Header.Set("Authorization", "Bearer "+tc.key)
			r.Header.Set("Origin", tc.origin)
			r.Header.Set("X-Forwarded-For", tc.forwarded)
			w := httptest.NewRecorder()
			tc.handler.ServeHTTP(w, r)
			if w.Code != tc.want {
				t.Fatalf("status %d, want %d: %s", w.Code, tc.want, w.Body.String())
			}
		})
	}
}

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
