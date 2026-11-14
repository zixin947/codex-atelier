package usage

// The workbench supplies a scoped snapshot to the original aggregators.
// Agent/model names alone never establish a ChatGPT account or upstream.
import (
	"slices"
	"time"
)

func AtelierChatGPTRecord(r Record) bool {
	return r.Provider == "codex" || r.Provider == UnknownProvider && r.Agent == "codex" && r.SessionOfficialLogin
}

func AtelierChatGPTLedger(p Period, zone *time.Location) Ledgered {
	l := LedgerOf(p, Filter{Zone: zone})
	out := Ledgered{Rows: []Row{}, Agents: []string{}, Providers: []string{}}
	for _, r := range l.Rows {
		if AtelierChatGPTRecord(r.Record) {
			out.Rows = append(out.Rows, r)
			if !r.IsRejected() {
				out.Sum.addRow(r)
			}
			if !slices.Contains(out.Agents, r.Agent) {
				out.Agents = append(out.Agents, r.Agent)
			}
			if !slices.Contains(out.Providers, r.Provider) {
				out.Providers = append(out.Providers, r.Provider)
			}
		}
	}
	return out
}

func AtelierChatGPTQuery(p Period, f Filter, offset, limit int) RequestPage {
	if limit <= 0 {
		limit = 100
	}
	return pageFromLedgerAt(p, f, max(0, offset), min(500, limit), AtelierChatGPTLedger(p, f.Zone), Clock().In(f.loc()))
}

func AtelierChatGPTSummary(p Period, zone *time.Location) Summary {
	if zone == nil {
		zone = time.Local
	}
	now := Clock().In(zone)
	var rows []Record
	Visit(p.Since(now), func(r Record) {
		if r.Provider == "codex" {
			rows = append(rows, r)
		}
	})
	return topOf(summarize(p, now, rows), 100)
}

func AtelierChatGPTHeatmap(f Filter) *Heatmap {
	f.Day = ""
	now := Clock().In(f.loc())
	l := AtelierChatGPTLedger(heatmapPeriod, f.Zone).Filtered(f)
	return heatmapOf(heatmapSince(now), now, func(add func(Row)) {
		for _, r := range l.Rows {
			add(r)
		}
	})
}

func AtelierChatGPTKeys() map[string]bool {
	out, foreign := map[string]bool{}, map[string]bool{}
	for _, r := range LedgerOf(All, Filter{}).Rows {
		if r.Agent == "codex" && r.Provider != UnknownProvider && r.Provider != "codex" {
			if r.Session != "" {
				foreign["codex:"+r.Session] = true
			}
			if r.NativeSession != "" {
				foreign["codex:"+r.NativeSession] = true
			}
		}
		if !AtelierChatGPTRecord(r.Record) {
			continue
		}
		if r.Agent == "codex" {
			if r.Session != "" {
				out["codex:"+r.Session] = true
			}
			if r.NativeSession != "" {
				out["codex:"+r.NativeSession] = true
			}
		}
	}
	for key := range foreign {
		delete(out, key)
	}
	return out
}

// Feed only ChatGPT gateway records to the original session accumulator.
func AtelierChatGPTGatewaySessions(since time.Time, nativeKeys map[string]bool) ([]GatewaySession, []GatewayDayUsage) {
	chunk := &rowChunk{}
	Visit(since, func(r Record) {
		if r.Provider == "codex" {
			chunk.add(Row{Record: r}, "", int64(len(chunk.Rows)), r.Failed())
		}
	})
	return buildGatewaySessions(&logSnapshot{blocks: []*rowChunk{chunk}}, since, nativeKeys, nil)
}
