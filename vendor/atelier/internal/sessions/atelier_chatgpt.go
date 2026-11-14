package sessions

import "time"

// Select the session files before the original statistics algorithm runs.
func AtelierChatGPTStats(days int, allowed map[string]bool) Stats {
	storeMu.RLock()
	defer storeMu.RUnlock()
	mu.Lock()
	defer mu.Unlock()
	loadCache()
	defer useDBs()()
	var files []file
	for _, f := range allFiles() {
		if f.agent == "codex" && allowed[f.key] {
			files = append(files, f)
		}
	}
	refresh(files, files)
	price, titles := pricer(), codexTitleOf()
	listed := map[string]Session{}
	folders := map[string]string{}
	for key, fs := range byKey(files, nil) {
		if s, ok := assembleListed(fs, price, titles); ok {
			listed[key], folders[key] = s, s.Cwd
		}
	}
	now := time.Now()
	today := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.Local)
	return statsOfRange(days, today, files, nil, nil, folders, listed, codexIdentitySnapshot(files), codexSubagentTaskTitles(files), price, titles)
}
