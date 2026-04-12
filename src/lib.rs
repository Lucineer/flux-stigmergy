pub mod environment;
pub mod trace;

pub use environment::SharedEnvironment;
pub use trace::{Trace, TraceType, Waypoint};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_and_read() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "key1", "val", 100, 500, TraceType::Info));
        let t = env.read("key1").unwrap();
        assert_eq!(t.value, "val");
        assert_eq!(t.reads, 1);
    }

    #[test]
    fn read_nonexistent() {
        let mut env = SharedEnvironment::new();
        assert!(env.read("nope").is_none());
    }

    #[test]
    fn read_all_prefix() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "loc:door", "open", 100, 500, TraceType::Info));
        env.deposit(Trace::new(1, "loc:window", "closed", 100, 500, TraceType::Info));
        env.deposit(Trace::new(1, "other", "x", 100, 500, TraceType::Info));
        let results = env.read_all("loc:", 10);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn modify() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "key1", "old", 100, 500, TraceType::Info));
        let ok = env.modify(1, "key1", "new", 100);
        assert!(ok);
        assert_eq!(env.read("key1").unwrap().value, "new");
        assert_eq!(env.read("key1").unwrap().strength, 600);
    }

    #[test]
    fn modify_nonexistent() {
        let mut env = SharedEnvironment::new();
        assert!(!env.modify(1, "nope", "x", 10));
    }

    #[test]
    fn erase_by_author() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "key1", "val", 100, 500, TraceType::Info));
        assert!(env.erase(1, "key1"));
        assert!(env.read("key1").is_none());
    }

    #[test]
    fn erase_wrong_author() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "key1", "val", 100, 500, TraceType::Info));
        assert!(!env.erase(2, "key1"));
    }

    #[test]
    fn decay() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "old", "val", 1000, 1000, TraceType::Info));
        env.read("old"); // boost reads
        let removed = env.decay(10, 50.0, 0, 1000); // half-life 10s, now=1000
        assert_eq!(removed, 0); // still above min
    }

    #[test]
    fn gc_removes_weak() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "weak", "val", 100, 5, TraceType::Info));
        env.deposit(Trace::new(1, "strong", "val", 100, 500, TraceType::Info));
        let removed = env.gc(10);
        assert_eq!(removed, 1);
        assert_eq!(env.stats().total_traces, 1);
    }

    #[test]
    fn by_author() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "a", "1", 100, 100, TraceType::Info));
        env.deposit(Trace::new(2, "b", "2", 100, 100, TraceType::Info));
        env.deposit(Trace::new(1, "c", "3", 100, 100, TraceType::Warning));
        assert_eq!(env.by_author(1).len(), 2);
        assert_eq!(env.by_author(2).len(), 1);
    }

    #[test]
    fn by_type() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "a", "1", 100, 100, TraceType::Claim));
        env.deposit(Trace::new(1, "b", "2", 100, 100, TraceType::Info));
        env.deposit(Trace::new(1, "c", "3", 100, 100, TraceType::Claim));
        assert_eq!(env.by_type(&TraceType::Claim).len(), 2);
    }

    #[test]
    fn strongest() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "a", "1", 100, 100, TraceType::Info));
        env.deposit(Trace::new(1, "b", "2", 100, 900, TraceType::Info));
        env.deposit(Trace::new(1, "c", "3", 100, 500, TraceType::Info));
        let s = env.strongest(2);
        assert_eq!(s[0].strength, 900);
        assert_eq!(s[1].strength, 500);
    }

    #[test]
    fn oldest() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "c", "3", 300, 100, TraceType::Info));
        env.deposit(Trace::new(1, "a", "1", 100, 100, TraceType::Info));
        env.deposit(Trace::new(1, "b", "2", 200, 100, TraceType::Info));
        let o = env.oldest(2);
        assert_eq!(o[0].timestamp, 100);
        assert_eq!(o[1].timestamp, 200);
    }

    #[test]
    fn waypoint_build_and_follow() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "wp:0", "start", 100, 800, TraceType::Waypoint));
        env.deposit(Trace::new(1, "wp:1", "middle", 100, 600, TraceType::Waypoint));
        env.deposit(Trace::new(1, "wp:2", "end", 100, 400, TraceType::Waypoint));
        let wp = Waypoint::from_trace_ids(1, vec![0, 1, 2]);
        let all_traces: Vec<Trace> = env.read_all("wp:", 10).into_iter().cloned().collect();
        let path = wp.follow(&all_traces);
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].value, "start");
        assert_eq!(path[2].value, "end");
    }

    #[test]
    fn stats() {
        let mut env = SharedEnvironment::new();
        env.deposit(Trace::new(1, "a", "1", 100, 100, TraceType::Info));
        env.deposit(Trace::new(1, "b", "2", 100, 200, TraceType::Warning));
        env.read("a");
        env.read("a");
        let s = env.stats();
        assert_eq!(s.total_traces, 2);
        assert_eq!(s.total_reads, 2);
        assert!((s.avg_strength - 150.0).abs() < 0.01);
        assert_eq!(s.by_type[0], 1); // Info
        assert_eq!(s.by_type[1], 1); // Warning
    }

    #[test]
    fn strength_capped_at_1000() {
        let t = Trace::new(1, "k", "v", 100, 5000, TraceType::Info);
        assert_eq!(t.strength, 1000);
    }
}
