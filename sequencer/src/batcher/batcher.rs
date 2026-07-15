use std::time::Duration;

pub struct Batcher{
    maxWaitTime: Duration,
    maxBatchSize: usize,
}

impl Batcher{
    pub fn new(maxWaitTime: Duration, maxBatchSize: usize)->Self{
        Self{
            maxWaitTime: maxWaitTime,
            maxBatchSize: maxBatchSize,
        }
    }
}