use serde_json::{json, Value};
use std::collections::BTreeMap;
#[derive(Default)]
pub(super) struct UsageHistory {
    last: u64,
    seen: bool,
    model: Option<String>,
    days: BTreeMap<String, u64>,
    models: BTreeMap<String, u64>,
    pub warnings: Vec<String>,
}
impl UsageHistory {
    pub fn context(&mut self, model: &str) {
        self.model = Some(model.to_owned());
    }
    pub fn record(&mut self, total: u64, timestamp: Option<&str>) {
        if self.seen && total < self.last {
            self.warnings
                .push("累计 Token 计数发生回退；回退点未作为新增用量归属模型".into());
        } else {
            let delta = total.saturating_sub(self.last);
            if delta > 0 {
                *self
                    .models
                    .entry(self.model.clone().unwrap_or_else(|| "未归属模型".into()))
                    .or_default() += delta;
            }
        }
        self.last = total;
        self.seen = true;
        if let Some(date) = timestamp.and_then(|t| {
            time::OffsetDateTime::parse(t, &time::format_description::well_known::Rfc3339).ok()
        }) {
            let day = date.to_offset(time::UtcOffset::UTC).date().to_string();
            self.days.insert(day, total);
        } else {
            self.warnings
                .push("部分用量记录缺少有效时间，未放入日期曲线".into());
        }
    }
    pub fn timeline(&self) -> Value {
        json!(self
            .days
            .iter()
            .map(|(date, total)| json!({"date":date,"total":total}))
            .collect::<Vec<_>>())
    }
    pub fn model_usage(&self) -> Value {
        let mut rows = self.models.iter().collect::<Vec<_>>();
        rows.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        json!(rows
            .into_iter()
            .map(|(model, total)| json!({"model":model,"total":total}))
            .collect::<Vec<_>>())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn groups_days_and_deduplicates_snapshots() {
        let mut h = UsageHistory::default();
        h.context("a");
        h.record(100, Some("2026-11-01T01:00:00Z"));
        h.record(100, Some("2026-11-01T02:00:00Z"));
        h.record(150, Some("2026-11-01T03:00:00Z"));
        h.context("b");
        h.record(220, Some("2026-11-02T01:00:00Z"));
        assert_eq!(
            h.timeline(),
            json!([{"date":"2026-11-01","total":150},{"date":"2026-11-02","total":220}])
        );
        assert_eq!(
            h.model_usage(),
            json!([{"model":"a","total":150},{"model":"b","total":70}])
        );
    }
    #[test]
    fn missing_model_and_counter_reset_are_not_misattributed() {
        let mut h = UsageHistory::default();
        h.record(100, None);
        h.context("b");
        h.record(20, None);
        h.record(30, None);
        assert_eq!(
            h.model_usage(),
            json!([{"model":"未归属模型","total":100},{"model":"b","total":10}])
        );
        assert!(h.warnings.len() > 0);
        assert_eq!(h.timeline(), json!([]));
    }
}
