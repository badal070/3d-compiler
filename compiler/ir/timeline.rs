pub mod timeline {
    use crate::ids::{TimelineId, MotionId};
    use crate::value::Time;

    #[derive(Debug, Clone)]
    pub struct Timeline {
        pub id: TimelineId,
        pub events: Vec<TimedEvent>,
    }

    #[derive(Debug, Clone)]
    pub struct TimedEvent {
        pub start: Time,
        pub duration: Time,
        pub motion: MotionId,
    }

    impl Timeline {
        pub fn new(id: TimelineId) -> Self {
            Self {
                id,
                events: Vec::new(),
            }
        }

        pub fn add_event(&mut self, event: TimedEvent) {
            self.events.push(event);
            // Keep events sorted by start time for efficient querying
            self.events.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
        }

        pub fn events_at(&self, time: Time) -> Vec<&TimedEvent> {
            self.events
                .iter()
                .filter(|e| {
                    let end = Time::seconds(e.start.value() + e.duration.value());
                    e.start <= time && time < end
                })
                .collect()
        }

        pub fn duration(&self) -> Time {
            self.events
                .iter()
                .map(|e| Time::seconds(e.start.value() + e.duration.value()))
                .max()
                .unwrap_or(Time::seconds(0.0))
        }
    }

    impl TimedEvent {
        pub fn new(start: Time, duration: Time, motion: MotionId) -> Self {
            Self { start, duration, motion }
        }

        pub fn end_time(&self) -> Time {
            Time::seconds(self.start.value() + self.duration.value())
        }

        pub fn is_active_at(&self, time: Time) -> bool {
            self.start <= time && time < self.end_time()
        }
    }
}