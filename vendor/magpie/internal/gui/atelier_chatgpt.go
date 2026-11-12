package gui

// ChatGPT-only endpoints adapt the original Usage page to the workbench.
import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"net/http"
	"sort"
	"strconv"
	"strings"
	"time"

	"github.com/yetone/magpie/internal/provider"
	"github.com/yetone/magpie/internal/sessions"
	"github.com/yetone/magpie/internal/settings"
	"github.com/yetone/magpie/internal/usage"
	"github.com/yetone/magpie/internal/usageapi"
)

const atelierChatGPTPrefix = "/api/atelier-chatgpt/"

func atelierChatGPT(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		path, scoped := strings.CutPrefix(r.URL.Path, atelierChatGPTPrefix)
		if !scoped {
			next.ServeHTTP(w, r)
			return
		}
		r = r.Clone(r.Context())
		r.URL.Path = "/api/" + path
		r.URL.RawPath = ""
		q := r.URL.Query()
		p, zone := usageapi.PeriodOf(q.Get("period")), usageapi.ZoneOf(r)
		filter := usageapi.FilterIn(r)
		ctx, cancel := context.WithTimeout(r.Context(), 30*time.Second)
		defer cancel()
		switch r.Method + " " + path {
		case "GET state":
			writeJSON(w, stateJSON{Agents: []agentJSON{}, Clients: []clientJSON{{ID: "codex", Name: "Codex", Icon: "codex-color"}}, Profiles: []profileJSON{}, Settings: settings.Shared()})
		case "GET usage":
			writeJSON(w, usageapi.AtelierChatGPTPage(p, zone))
		case "GET usage/requests":
			offset, _ := strconv.Atoi(q.Get("offset"))
			limit, _ := strconv.Atoi(q.Get("limit"))
			writeJSON(w, usageapi.AtelierChatGPTLedger(p, filter, offset, limit))
		case "GET usage/heatmap":
			writeJSON(w, usage.AtelierChatGPTHeatmap(filter))
		case "GET usage/requests.csv":
			rows := usage.AtelierChatGPTLedger(p, zone).Filtered(filter).Rows
			w.Header().Set("Content-Type", "text/csv; charset=utf-8")
			w.Header().Set("Content-Disposition", `attachment; filename="chatgpt-requests.csv"`)
			usage.WriteCSV(w, rows)
		case "POST usage/requests/export":
			rows := usage.AtelierChatGPTLedger(p, zone).Filtered(filter).Rows
			name, err := saveRequestsCSV(rows, "chatgpt-"+csvStamp(p, q.Get("day"), usage.Clock()))
			if err != nil {
				fail(w, err)
				return
			}
			writeJSON(w, map[string]any{"path": tilde(name), "rows": len(rows)})
		case "GET usage/quotas", "POST usage/quotas/refresh":
			if id := q.Get("provider"); id != "" && id != "codex" {
				http.Error(w, "仅支持 ChatGPT 账号", http.StatusForbidden)
				return
			}
			if r.Method == "POST" {
				provider.RefreshUsage(ctx, "codex", q.Get("user"))
			}
			cards := []provider.SubscriptionQuota{}
			for user, card := range provider.LoginUsage(ctx, "codex") {
				card.User, card.Name, card.Icon = user, "ChatGPT", "codex-color"
				cards = append(cards, card)
			}
			sort.Slice(cards, func(i, j int) bool { return cards[i].User < cards[j].User })
			writeJSON(w, withTrayKinds(provider.WithHeldWindows(provider.WithWindowStarts(usage.WithWindowHolds(provider.InProviderOrder(cards), usage.Clock()), usage.Clock()), usage.Clock())))
		case "GET usage/quotas/history":
			writeJSON(w, provider.QuotaHistories(provider.QuotaHistorySince(q.Get("days"), time.Now()), "codex", ""))
		case "GET usage/quotas/history/holds":
			from, e1 := time.Parse(time.RFC3339Nano, q.Get("from"))
			to, e2 := time.Parse(time.RFC3339Nano, q.Get("to"))
			now := usage.Clock()
			if e1 != nil || e2 != nil || !from.Before(to) || to.After(now) || q.Get("provider") != "codex" || q.Get("user") == "" {
				http.Error(w, "ChatGPT account and a completed time range are required", http.StatusBadRequest)
				return
			}
			cards := []provider.SubscriptionQuota{}
			for _, card := range provider.CachedCards(now) {
				if card.Provider == "codex" {
					cards = append(cards, card)
				}
			}
			hist := provider.QuotaHistories(now.Add(-45*24*time.Hour), "codex", q.Get("user"))
			writeJSON(w, usage.HistoricalWindowHolds(cards, hist, from, to))
		case "GET tune":
			days, _ := strconv.Atoi(q.Get("days"))
			if days <= 0 || days > 30 {
				days = 7
			}
			allowed := usage.AtelierChatGPTKeys()
			calls := []sessions.Call{}
			for _, call := range sessions.Calls(time.Now().AddDate(0, 0, -days)) {
				if call.Agent == "codex" && allowed["codex:"+call.Session] {
					calls = append(calls, call)
				}
			}
			out := tuneOf(calls, days, sessions.CodexDir())
			kept := out.Agents[:0]
			for _, a := range out.Agents {
				if a.Agent == "codex" {
					kept = append(kept, a)
				}
			}
			out.Agents = kept
			writeJSON(w, out)
		case "POST usage/clear":
			var in struct{ Provider, Model string }
			if err := json.NewDecoder(r.Body).Decode(&in); err != nil {
				fail(w, err)
				return
			}
			if in.Provider != "" && in.Provider != "codex" {
				http.Error(w, "仅支持 ChatGPT 用量", http.StatusForbidden)
				return
			}
			if err := settings.ClearUsage("codex", in.Model, usage.Clock()); err != nil {
				fail(w, err)
				return
			}
			writeJSON(w, map[string]bool{"ok": true})
		case "GET usage/traffic":
			// The original process-wide counter has no provider dimension.
			// An unavailable value must not expose another platform's traffic.
			writeJSON(w, trafficJSON{})
		case "GET settings", "GET model-price", "POST model-price", "POST usage/arrange":
			next.ServeHTTP(w, r)
		default:
			if atelierChatGPTSessions(w, r, path, next) {
				return
			}
			// Codex-specific account controls retain their original backend.
			if strings.HasPrefix(path, "settings/codex-") || path == "usage/codex-reset" || path == "tune/apply" || path == "tune/undo" || path == "warm-now" {
				body, err := io.ReadAll(io.LimitReader(r.Body, 1<<20))
				if err != nil {
					fail(w, err)
					return
				}
				var in struct{ Provider, Agent string }
				if len(body) > 0 {
					if err := json.Unmarshal(body, &in); err != nil {
						fail(w, err)
						return
					}
				}
				if (strings.HasPrefix(path, "tune/") || path == "warm-now") && in.Agent != "codex" {
					http.Error(w, "仅支持 Codex 上下文设置", http.StatusForbidden)
					return
				}
				if in.Provider != "" && in.Provider != "codex" {
					http.Error(w, "仅支持 ChatGPT 账号", http.StatusForbidden)
					return
				}
				r.Body = io.NopCloser(bytes.NewReader(body))
				next.ServeHTTP(w, r)
				return
			}
			http.NotFound(w, r)
		}
	})
}
