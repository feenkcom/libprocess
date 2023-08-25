#[derive(Clone, Debug)]
pub struct SemaphoreSignaller {
    semaphore_callback: unsafe extern "C" fn(usize),
    semaphore_index: usize,
}

impl SemaphoreSignaller {
    pub fn new(semaphore_callback: unsafe extern "C" fn(usize), semaphore_index: usize) -> Self {
        Self {
            semaphore_callback,
            semaphore_index,
        }
    }

    pub fn signal(&self) {
        let callback = self.semaphore_callback;
        unsafe { callback(self.semaphore_index) };
    }
}
