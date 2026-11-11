// Atelier-owned host for the unmodified magpie protocol and routing core.
// Upstream: e6f151507f3c70f483d4bd476fa69faef155fe8c, MIT, Copyright (c) 2026 yetone.
package main

import (
	"bufio"
	"bytes"
	"context"
	"crypto/subtle"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"

	"github.com/yetone/magpie/internal/appdir"
	"github.com/yetone/magpie/internal/catalog"
	"github.com/yetone/magpie/internal/gateway"
	"github.com/yetone/magpie/internal/gui"
	"github.com/yetone/magpie/internal/provider"
	"github.com/yetone/magpie/internal/routes"
	"github.com/yetone/magpie/internal/settings"
	"github.com/yetone/magpie/internal/steady"
)

type config struct {
	OriginalUI bool   `json:"originalUI"`
	PrepareURL string `json:"prepareURL"`
	PrepareKey string `json:"prepareKey"`

	DefaultProvider string `json:"defaultProvider"`
	DefaultModel    string `json:"defaultModel"`
	DefaultRoute    string `json:"defaultRoute"`

	Root             string              `json:"root"`
	Port             int                 `json:"port"`
	Key              string              `json:"key"`
	Providers        []provider.Provider `json:"providers"`
	Groups           []provider.Group    `json:"groups"`
	Settings         settings.Settings   `json:"settings"`
	DiscoverAccounts bool                `json:"discoverAccounts"`
}

func configure(c config) error {
	if !filepath.IsAbs(c.Root) || c.Port < 0 || c.Port > 65535 || len(c.Key) < 32 {
		return errors.New("invalid Atelier router configuration")
	}
	// A dedicated portable root keeps all core state out of installed magpie.
	if err := os.MkdirAll(filepath.Join(c.Root, "data"), 0700); err != nil {
		return err
	}
	if err := os.WriteFile(filepath.Join(c.Root, ".portable"), nil, 0600); err != nil {
		return err
	}
	appdir.UseExecutable(filepath.Join(c.Root, "atelier-router"))
	if !c.DiscoverAccounts {
		os.Setenv("MAGPIE_NO_ACCOUNTS", "1")
	}
	if c.OriginalUI {
		c.Settings = settings.Load()
	}
	c.Settings.Port = c.Port
	c.Settings.LAN = false
	c.Settings.LANKey = c.Key
	if !c.OriginalUI {
		c.Settings.RequestArchive = false
		c.Settings.RequestArchiveLocal = false
		c.Settings.GatewayConversations = false
	}
	if err := settings.Save(c.Settings); err != nil {
		return err
	}
	if c.OriginalUI {
		return nil
	}
	// Replace the complete managed set, so removed connections cannot survive.
	initial, err := json.Marshal(map[string]any{"providers": []provider.Provider{}, "groups": []provider.Group{}, "noAutoGroups": true})
	if err != nil {
		return err
	}
	if err := steady.WriteFile(provider.Path(), initial, 0600); err != nil {
		return err
	}
	for _, p := range c.Providers {
		if err := provider.Save(p); err != nil {
			return err
		}
	}
	for _, g := range c.Groups {
		if err := provider.SaveGroup(g); err != nil {
			return err
		}
	}
	return nil
}

// Only inference protocols, catalogs and routing observations are exposed.
// Upstream process-update, credential issuance and operational control remain
// Atelier's responsibility and cannot be reached with an inference key.
func protocolPath(p string) bool {
	// Core inference, tool and catalog routes pass through without a second
	// allowlist drifting behind upstream. Process/key administration stays local.
	for _, blocked := range []string{gateway.KeysPath, gateway.KeyPagePath, gateway.RestartPath, gateway.CPUPath, gateway.HeapPath, gateway.GoroutinesPath, provider.RemoteUpdatePath, provider.RemoteCodexResetPath} {
		if p == blocked || strings.HasPrefix(p, blocked+"/") {
			return false
		}
	}
	return true
}

func inferenceAccess(c config, address string, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/health" {
			w.Header().Set("Content-Type", "application/json")
			io.WriteString(w, `{"status":"ok","service":"codex-atelier"}`)
			return
		}
		key := strings.TrimPrefix(r.Header.Get("Authorization"), "Bearer ")
		if key == "" {
			key = r.Header.Get("x-api-key")
		}
		if key == "" {
			key = r.Header.Get("x-goog-api-key")
		}
		if key == "" {
			key = r.URL.Query().Get("key")
		}
		// Native agents use upstream's loopback tokens and caller-key rules.
		// The legacy Atelier connection still requires its private process key.
		if r.Header.Get("Origin") != "" || !c.OriginalUI && subtle.ConstantTimeCompare([]byte(key), []byte(c.Key)) != 1 {
			http.Error(w, "Atelier gateway authentication required", http.StatusUnauthorized)
			return
		}
		if r.Host != address && r.Host != strings.Replace(address, "127.0.0.1", "localhost", 1) {
			http.Error(w, "invalid gateway host", http.StatusForbidden)
			return
		}
		if !protocolPath(r.URL.Path) {
			http.NotFound(w, r)
			return
		}
		next.ServeHTTP(w, r)
	})
}

func run() error {
	decoder := json.NewDecoder(bufio.NewReader(io.LimitReader(os.Stdin, 16<<20)))
	var c config
	if err := decoder.Decode(&c); err != nil {
		return err
	}
	if err := configure(c); err != nil {
		return err
	}
	for name, strategy := range routes.Builtin {
		routes.Register(name, strategy)
	}
	provider.ServeGroups()
	catalog.Seed()
	core := gateway.New()
	listener, err := net.Listen("tcp", fmt.Sprintf("127.0.0.1:%d", c.Port))
	if err != nil {
		return err
	}
	address := "http://" + listener.Addr().String()
	handler := core.Handler()
	if c.OriginalUI {
		handler = core.AtelierHandler()
	}
	srv := &http.Server{ReadHeaderTimeout: 30 * time.Second, Handler: inferenceAccess(c, listener.Addr().String(), http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Preserve Atelier's selected route for unqualified JSON model names.
		if strings.Contains(r.Header.Get("Content-Type"), "application/json") && r.Body != nil && r.Method == http.MethodPost {
			body, readErr := io.ReadAll(io.LimitReader(r.Body, 16<<20+1))
			if readErr != nil || len(body) > 16<<20 {
				http.Error(w, "request body too large or incomplete", http.StatusRequestEntityTooLarge)
				return
			}
			var value map[string]json.RawMessage
			if json.Unmarshal(body, &value) == nil {
				var model string
				if json.Unmarshal(value["model"], &model) == nil && model != "" && !strings.Contains(model, "/") && c.DefaultProvider != "" {
					if model == c.DefaultModel && c.DefaultRoute != "" {
						model = c.DefaultRoute
					} else {
						model = c.DefaultProvider + "/" + model
					}
					value["model"], _ = json.Marshal(model)
					body, _ = json.Marshal(value)
				}
			}
			if c.PrepareURL != "" && value != nil {
				prepare, err := http.NewRequestWithContext(r.Context(), http.MethodPost, c.PrepareURL, bytes.NewReader(body))
				if err != nil {
					http.Error(w, "Atelier preparation unavailable", 503)
					return
				}
				prepare.Header.Set("Authorization", "Bearer "+c.PrepareKey)
				prepare.Header.Set("Content-Type", "application/json")
				client := http.Client{Timeout: 10 * time.Second}
				prepared, err := client.Do(prepare)
				if err != nil {
					http.Error(w, "Atelier preparation unavailable", 503)
					return
				}
				result, readErr := io.ReadAll(io.LimitReader(prepared.Body, 16<<20+1))
				prepared.Body.Close()
				if prepared.StatusCode != 200 || readErr != nil || len(result) > 16<<20 {
					http.Error(w, "Atelier preparation failed", 503)
					return
				}
				body = result
			}
			r.Body = io.NopCloser(bytes.NewReader(body))
			r.ContentLength = int64(len(body))
		}
		handler.ServeHTTP(w, r)
	}))}
	var output sync.Mutex
	send := func(v any) { output.Lock(); defer output.Unlock(); json.NewEncoder(os.Stdout).Encode(v) }
	ui, closeUI, uiErr := gui.AtelierUI(core, c.Key)
	if uiErr != nil {
		return uiErr
	}
	defer closeUI()
	send(map[string]any{"ready": true, "address": address + "/v1", "managementURL": ui})
	stopped := make(chan struct{})
	go func() { defer close(stopped); err = srv.Serve(listener) }()
	ticker := time.NewTicker(time.Second)
	defer ticker.Stop()
	inputEnded := make(chan struct{})
	go func() { var ignored any; decoder.Decode(&ignored); close(inputEnded) }()
	for {
		select {
		case <-ticker.C:
			send(map[string]any{"trace": core.Trace(context.Background(), 0, 0)})
		case <-inputEnded:
			ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
			defer cancel()
			shutdownErr := core.AtelierShutdown(ctx, srv)
			send(map[string]any{"trace": core.Trace(context.Background(), 0, 0)})
			if shutdownErr != nil {
				return shutdownErr
			}
			return nil
		case <-stopped:
			if errors.Is(err, http.ErrServerClosed) {
				return nil
			}
			return err
		}
	}
}

func main() {
	log.SetOutput(io.Discard) // Upstream logs may contain prompts; never forward them to IPC.
	if len(os.Args) == 2 && os.Args[1] == "--capabilities" {
		strategies := []string{"smart", "manual"}
		for k := range routes.Builtin {
			strategies = append(strategies, k)
		}
		sort.Strings(strategies)
		json.NewEncoder(os.Stdout).Encode(map[string]any{"upstream": "e6f151507f3c70f483d4bd476fa69faef155fe8c", "strategies": strategies, "protocols": []string{"chat", "responses", "responses-websocket", "anthropic", "gemini", "ollama", "images", "audio", "videos", "embeddings", "rerank", "decisions", "native", "mcp", "codex-native", "search"}})
		return
	}
	if err := run(); err != nil {
		json.NewEncoder(os.Stdout).Encode(map[string]any{"error": err.Error()})
		os.Exit(1)
	}
}
