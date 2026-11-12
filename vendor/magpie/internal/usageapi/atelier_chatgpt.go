package usageapi

import (
	"time"

	"github.com/yetone/magpie/internal/usage"
)

func AtelierChatGPTPage(p usage.Period, zone *time.Location) Usage {
	return stateOf(usage.AtelierChatGPTSummary(p, zone))
}

func AtelierChatGPTLedger(p usage.Period, f usage.Filter, offset, limit int) Ledger {
	return ledgerPageOf(p, f, offset, usage.AtelierChatGPTQuery(p, f, offset, limit))
}
