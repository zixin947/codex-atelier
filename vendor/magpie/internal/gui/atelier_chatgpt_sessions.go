package gui

import (
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/yetone/magpie/internal/sessions"
	"github.com/yetone/magpie/internal/usage"
)

func atelierChatGPTSessions(w http.ResponseWriter, r *http.Request, path string, next http.Handler) bool {
	if !strings.HasPrefix(path, "sessions") && !strings.HasPrefix(path, "context") && path != "archive" && !strings.HasPrefix(path, "archive/") && !strings.HasPrefix(path, "gateway/") && path != "usage/requests/content" {
		return false
	}
	q := r.URL.Query()
	allowed := usage.AtelierChatGPTKeys()
	days, _ := strconv.Atoi(q.Get("days"))
	switch r.Method + " " + path {
	case "GET sessions":
		out := sessionsJSON{Sessions: []sessionJSON{}, Dirs: []string{}}
		native, keys := []sessions.Session{}, map[string]bool{}
		for _, s := range sessions.List(sessions.All) {
			if s.Agent == "codex" && allowed[s.Agent+":"+s.ID] {
				native = append(native, s)
				for _, key := range sessions.GatewayKeys(s) {
					keys[key] = true
				}
			}
		}
		gs, _ := usage.AtelierChatGPTGatewaySessions(time.Time{}, keys)
		ext := []sessions.ExternalSession{}
		for _, g := range gs {
			ext = append(ext, gatewayListed(g, sessions.GatewayTitle))
		}
		for _, s := range sessions.MergeExternal(native, ext) {
			name, icon := s.Agent, "generic"
			if s.Agent == "codex" {
				name, icon = "Codex", "codex-color"
			}
			s.Path = tilde(s.Path)
			out.Sessions = append(out.Sessions, sessionJSON{Session: s, Name: name, Icon: icon})
		}
		writeJSON(w, out)
	case "GET sessions/stats":
		w.Header().Set("Content-Type", "application/json")
		if err := statsAnswer(w, atelierChatGPTStats(max(0, days), allowed), map[string]string{"codex": "Codex"}); err != nil {
			fail(w, err)
		}
	case "GET sessions/overview":
		writeJSON(w, atelierChatGPTStats(max(0, days), allowed).Overview(q.Get("agent"), q.Get("model"), q.Get("cwd"), 8))
	case "GET sessions/progress":
		next.ServeHTTP(w, r)
	case "GET sessions/one":
		if !allowed[q.Get("key")] {
			gs, _ := usage.AtelierChatGPTGatewaySessions(time.Time{}, nil)
			for _, g := range gs {
				if g.Agent+":"+g.ID == q.Get("key") {
					writeJSON(w, sessionJSON{Session: gatewaySession(g), Name: g.Agent, Icon: "generic"})
					return true
				}
			}
			http.NotFound(w, r)
			return true
		}
		next.ServeHTTP(w, r)
	case "GET sessions/transcript", "GET sessions/markdown", "GET usage/requests/content":
		id := q.Get("id")
		if path == "usage/requests/content" {
			id = q.Get("session")
		}
		if q.Get("agent") != "codex" || !allowed["codex:"+id] {
			http.NotFound(w, r)
			return true
		}
		next.ServeHTTP(w, r)
	case "GET context", "GET context/session", "GET context/text", "GET context/reply", "GET gateway/call", "GET archive", "GET archive/file", "POST archive/export":
		atelierChatGPTContext(w, r, path, allowed, days, next)
	default:
		http.NotFound(w, r)
	}
	return true
}
func atelierChatGPTContext(w http.ResponseWriter, r *http.Request, path string, allowed map[string]bool, days int, next http.Handler) {
	q := r.URL.Query()
	if days <= 0 || days > 30 {
		days = 7
	}
	routes, archives, sessionKeys := map[int64]bool{}, map[string]bool{}, map[string]bool{}
	foreign := map[int64]bool{}
	for _, row := range usage.LedgerOf(usage.All, usage.Filter{}).Rows {
		if row.RouteID != 0 && row.Provider != "codex" {
			foreign[row.RouteID] = true
		}
	}
	for _, row := range usage.AtelierChatGPTLedger(usage.All, nil).Rows {
		if foreign[row.RouteID] {
			continue
		}
		if row.Session != "" {
			sessionKeys[row.Agent+":"+row.Session] = true
		}
		if row.RouteID != 0 {
			routes[row.RouteID] = true
		}
		if row.Archive != "" {
			archives[row.Archive] = true
		}
	}
	if path == "context" || path == "context/session" {
		if path == "context/session" && (!allowed[q.Get("agent")+":"+q.Get("id")] && !sessionKeys[q.Get("agent")+":"+q.Get("id")]) {
			http.NotFound(w, r)
			return
		}
		if path == "context/session" {
			days = ctxSessionDays
		}
		recs, _ := contextRecs(r, days)
		kept := make([]ctxRec, 0, len(recs))
		for _, rec := range recs {
			if routes[rec.ID] {
				kept = append(kept, rec)
			}
		}
		if path == "context" {
			writeJSON(w, contextOfRecs(kept, days))
		} else {
			out := contextOfRecsFor(kept, days, q.Get("agent")+"\x00"+q.Get("id"))
			var session *ctxSession
			if len(out.Sessions) > 0 {
				session = &out.Sessions[0]
			}
			writeJSON(w, map[string]any{"session": session})
		}
		return
	}
	if strings.HasPrefix(path, "archive") {
		if !archives[q.Get("date")+"/"+q.Get("id")] {
			http.NotFound(w, r)
			return
		}
	} else {
		id, _ := strconv.ParseInt(q.Get("id"), 10, 64)
		if !routes[id] {
			http.NotFound(w, r)
			return
		}
	}
	next.ServeHTTP(w, r)
}
func atelierChatGPTStats(days int, allowed map[string]bool) sessions.Stats {
	st := sessions.AtelierChatGPTStats(days, allowed)
	keys := map[string]bool{}
	for key := range allowed {
		a, id, _ := strings.Cut(key, ":")
		keys[a+"|"+id] = true
	}
	var since time.Time
	if st.From != "" {
		since, _ = time.ParseInLocation(time.DateOnly, st.From, time.Local)
	}
	gs, gd := usage.AtelierChatGPTGatewaySessions(since, keys)
	ext, daily := []sessions.ExternalSession{}, []sessions.ExternalDayUsage{}
	for _, g := range gs {
		e := gatewayListed(g, sessions.GatewayTitle)
		e.Daily = gatewayDays(g)
		ext = append(ext, e)
	}
	for _, d := range gd {
		daily = append(daily, sessions.ExternalDayUsage{Date: d.Date, Agent: d.Agent, Model: d.Model, Tokens: d.Tokens, Cost: d.Cost, Priced: d.Priced})
	}
	return sessions.MergeExternalStats(st, daily, ext)
}
