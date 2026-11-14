package gui

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"context"
	"github.com/zixin947/codex-atelier/router/internal/provider"
	"github.com/zixin947/codex-atelier/router/internal/sessions"
	"github.com/zixin947/codex-atelier/router/internal/usage"
)

func TestAtelierChatGPTNativeSessionFilesAreScopedBeforeStatistics(t *testing.T) {
	sandboxHome(t)
	sessions.Reset()
	t.Setenv("CODEX_HOME", filepath.Join(os.Getenv("HOME"), ".codex"))
	t.Cleanup(sessions.Reset)
	at := time.Now().Add(-time.Hour)
	home, _ := os.UserHomeDir()
	good := "0190aaaa-1111-7222-8333-444455556666"
	other := "0190bbbb-1111-7222-8333-444455556666"
	writeCodexAPITestThread(t, filepath.Join(home, ".codex"), good, "", "", []codexTestCall{{at: at, model: "gpt-chatgpt", input: 10, output: 2}})
	writeCodexAPITestThread(t, filepath.Join(home, ".codex"), other, "", "", []codexTestCall{{at: at, model: "third-party-model", input: 1000, output: 200}})
	usage.Append(usage.Record{Time: at, Agent: "codex", Provider: "codex", Model: "gpt-chatgpt", Session: good, RouteID: 1, Archive: "2026-11-11/chatgpt", Input: 10, Status: 200})
	usage.Append(usage.Record{Time: at, Agent: "codex", Provider: "relay", Model: "third-party-model", Session: other, RouteID: 2, Input: 1000, Status: 200})
	h := atelierChatGPT(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(200) }))
	for _, path := range []string{"sessions", "sessions/stats", "sessions/overview"} {
		w := httptest.NewRecorder()
		h.ServeHTTP(w, httptest.NewRequest("GET", atelierChatGPTPrefix+path, nil))
		if w.Code != 200 || strings.Contains(w.Body.String(), other) || strings.Contains(w.Body.String(), "third-party-model") {
			t.Fatalf("%s leaked an excluded session: %d %s", path, w.Code, w.Body.String())
		}
		if path == "sessions" {
			var out sessionsJSON
			if err := json.Unmarshal(w.Body.Bytes(), &out); err != nil || len(out.Sessions) != 1 || out.Sessions[0].ID != good {
				t.Fatalf("scoped session list: %s", w.Body.String())
			}
		}
	}
	for _, path := range []string{"context/text?id=2", "archive?date=2026-11-11&id=other", "sessions/one?key=codex:" + other, "context/session?agent=claude&id=" + good} {
		w := httptest.NewRecorder()
		h.ServeHTTP(w, httptest.NewRequest("GET", atelierChatGPTPrefix+path, nil))
		if w.Code != 404 {
			t.Fatalf("unscoped resource reached original backend: %s: %d", path, w.Code)
		}
	}
}

func TestAtelierChatGPTDoesNotDelegateOtherPlatformWrites(t *testing.T) {
	h := atelierChatGPT(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		t.Fatal("foreign operation reached original backend")
	}))
	for _, tc := range []struct{ path, body string }{
		{"usage/quotas/refresh?provider=claude", "{}"},
		{"usage/clear", `{"provider":"claude"}`},
		{"usage/codex-reset", `{"Provider":"remote/claude"}`},
		{"tune/apply", `{"Agent":"claude"}`},
	} {
		w := httptest.NewRecorder()
		h.ServeHTTP(w, httptest.NewRequest("POST", atelierChatGPTPrefix+tc.path, strings.NewReader(tc.body)))
		if w.Code != 403 {
			t.Fatalf("%s: got %d, want 403", tc.path, w.Code)
		}
	}
}

func TestAtelierChatGPTQuotaCardsKeepAccountIdentity(t *testing.T) {
	sandboxHome(t)
	provider.LoginUsageVia(func(_ context.Context, agent string) map[string]provider.SubscriptionQuota {
		if agent != "codex" {
			t.Fatalf("foreign allowance queried: %s", agent)
		}
		return map[string]provider.SubscriptionQuota{"fixture@example.invalid": {Provider: "codex", Windows: []provider.QuotaWindow{}}}
	})
	t.Cleanup(func() { provider.LoginUsageVia(nil) })
	h := atelierChatGPT(http.NotFoundHandler())
	w := httptest.NewRecorder()
	h.ServeHTTP(w, httptest.NewRequest("GET", atelierChatGPTPrefix+"usage/quotas", nil))
	var out []provider.SubscriptionQuota
	if err := json.Unmarshal(w.Body.Bytes(), &out); err != nil || len(out) != 1 || out[0].User != "fixture@example.invalid" || out[0].Provider != "codex" {
		t.Fatalf("card identity lost: %s", w.Body.String())
	}
}
