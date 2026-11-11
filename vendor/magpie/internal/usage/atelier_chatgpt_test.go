package usage

import (
	"testing"
	"time"
)

func TestAtelierChatGPTScopePrecedesAggregationAndPaging(t *testing.T) {
	pageHome(t)
	now := holdClock(t, time.Date(2026, 11, 11, 12, 0, 0, 0, time.UTC))
	for _, r := range []Record{
		{Agent: "codex", Provider: "codex", Model: "o3", Input: 10, Session: "a"},
		{Agent: "codex", Provider: "codex", Model: "gpt", Input: 20, Session: "b"},
		{Agent: "codex", Provider: "claude", Model: "gpt-alias", Input: 1000},
		{Agent: "codex", Provider: "openai", Model: "gpt", Input: 2000},
		{Agent: "codex", Provider: UnknownProvider, Model: "gpt", Input: 4000},
	} {
		r.Time, r.Status = now.Add(-time.Minute), 200
		Append(r)
	}
	p := AtelierChatGPTQuery(Today, Filter{Zone: time.UTC}, 0, 1)
	if p.Total != 2 || len(p.Rows) != 1 || p.Sum.Calls != 2 || p.Sum.Input != 30 {
		t.Fatalf("scope applied after totals or paging: %+v", p)
	}
	for _, group := range p.By["provider"] {
		if group.ID != "codex" {
			t.Fatalf("foreign provider in facets: %+v", group)
		}
	}
	if p := AtelierChatGPTQuery(Today, Filter{Provider: "claude"}, 0, 100); p.Total != 0 {
		t.Fatal("a caller filter escaped the workbench scope")
	}
	s := AtelierChatGPTSummary(Today, time.UTC)
	if s.Calls != 2 || s.Input != 30 {
		t.Fatalf("summary mixed platforms: %+v", s)
	}
	h := AtelierChatGPTHeatmap(Filter{Zone: time.UTC})
	if len(h.Days) != 1 || h.Days[0].Calls != 2 {
		t.Fatalf("heatmap mixed platforms: %+v", h)
	}
}

func TestAtelierChatGPTNativeIdentityDoesNotOverrideKnownThirdParty(t *testing.T) {
	if !AtelierChatGPTRecord(Record{Agent: "codex", Provider: UnknownProvider, SessionOfficialLogin: true}) {
		t.Fatal("explicit native ChatGPT identity was excluded")
	}
	if AtelierChatGPTRecord(Record{Agent: "codex", Provider: "relay", SessionOfficialLogin: true}) {
		t.Fatal("session creator was confused with the upstream billing account")
	}
}

func TestAtelierChatGPTMixedNativeSessionIsNotCountedWhole(t *testing.T) {
	pageHome(t)
	now := holdClock(t, time.Now())
	Append(Record{Time: now, Agent: "codex", Provider: "codex", Session: "mixed", Status: 200})
	Append(Record{Time: now, Agent: "codex", Provider: "relay", Session: "mixed", Status: 200})
	if AtelierChatGPTKeys()["codex:mixed"] {
		t.Fatal("mixed-platform native file was admitted whole")
	}
}
