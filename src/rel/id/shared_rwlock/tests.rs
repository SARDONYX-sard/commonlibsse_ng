use crate::rel::id::shared_rwlock::SharedRwLock;
use std::sync::OnceLock;
use std::thread;
use windows::core::h;

//  50_000:   8.55s
// 100_000:  17.75s
const THREAD_COUNT: Primitive = 100_000;
type Primitive = usize;
static GLOBAL_SHARED_MEM: OnceLock<SharedRwLock<Primitive>> = OnceLock::new();

fn get_shared_memory() -> &'static SharedRwLock<Primitive> {
    GLOBAL_SHARED_MEM.get_or_init(|| SharedRwLock::new(h!("GlobalTest"), 1).unwrap().0)
}

#[test]
fn test_shared_memory_rwlock() {
    let shared_mem = get_shared_memory();

    let reader_handles: Vec<_> = (0..THREAD_COUNT)
        .map(|_| {
            thread::spawn(|| {
                let read_guard = get_shared_memory().read().unwrap();
                tracing::trace!("{}", read_guard[0]);
            })
        })
        .collect();

    let writer_handles: Vec<_> = (0..THREAD_COUNT)
        .map(|_| {
            thread::spawn(|| {
                let mut write_guard = get_shared_memory().write().unwrap();
                write_guard[0] += 1;
            })
        })
        .collect();

    for handle in reader_handles {
        handle.join().unwrap();
    }
    for handle in writer_handles {
        handle.join().unwrap();
    }

    assert_eq!(shared_mem.read().unwrap()[0], THREAD_COUNT);
}
