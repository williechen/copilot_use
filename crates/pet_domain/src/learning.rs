//! 複習排程規則；時間由呼叫端提供，規則不讀取系統時鐘或資料庫。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Schedule {
    pub repetitions: i64,
    pub interval_days: i64,
    pub ease: f64,
    pub due_at: i64,
}
impl Schedule {
    pub fn review(&mut self, quality: u8, at: i64) -> Result<(), &'static str> {
        if quality > 5 {
            return Err("評分必須介於 0 到 5");
        }
        if quality < 3 {
            self.repetitions = 0;
            self.interval_days = 1;
        } else {
            self.interval_days = match self.repetitions {
                0 => 1,
                1 => 6,
                _ => ((self.interval_days as f64) * self.ease)
                    .round()
                    .clamp(1.0, 3650.0) as i64,
            };
            self.repetitions += 1;
        }
        let delta = 5.0 - quality as f64;
        self.ease = (self.ease + 0.1 - delta * (0.08 + delta * 0.02)).clamp(1.3, 3.0);
        self.due_at = at + self.interval_days * 86400;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_quality_preserves_schedule_and_failure_resets_interval() {
        let mut schedule = Schedule {
            repetitions: 2,
            interval_days: 6,
            ease: 2.5,
            due_at: 42,
        };
        let before = schedule;
        assert!(schedule.review(6, 100).is_err());
        assert_eq!(schedule, before);
        schedule.review(5, 100).unwrap();
        assert_eq!(
            (
                schedule.repetitions,
                schedule.interval_days,
                schedule.due_at
            ),
            (3, 15, 100 + 15 * 86400)
        );
        schedule.review(0, 200).unwrap();
        assert_eq!(
            (
                schedule.repetitions,
                schedule.interval_days,
                schedule.due_at
            ),
            (0, 1, 200 + 86400)
        );
        for _ in 0..10 {
            schedule.review(0, 200).unwrap();
        }
        assert_eq!(schedule.ease, 1.3);
    }
}
