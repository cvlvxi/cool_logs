mod strategies;

use crate::logging::LinePartStrategy;

struct Adb<S: LinePartStrategy> {
    strategy: S
}

impl<S: LinePartStrategy> Adb<S> {
    fn new(strategy: S) -> Self {
        Self {
            strategy
        }
    } 
}