extern crate slog_async;
extern crate slog_term;

use slog:: {
    Drain,
    Logger
};
use self::slog_async:: {
   Async,
   AsyncGuard
};
use self::slog_term:: {
    FullFormat,
    TermDecorator
};

// Log structure containing a logger and asynchronous guard
pub struct Log {
    pub guard:  AsyncGuard,
    pub logger: Logger
}

// Log implementation
impl Log {
    /// constructs a new Log instance with a logger and asynchronous guard.
    /// care should be taken to drop the logger (and therefore the guard)
    /// before exiting / panicing to avoid mangling debug messages.
    ///
    /// # Example
    ///
    /// ```
    /// let log = logger::Log::new()
    /// debug!(log.logger, "debug message")
    /// drop(log)
    /// panic!("panic message")
    /// ```
    pub fn new() -> Self {
        let decor = TermDecorator::new().build();
        let drain = FullFormat::new(decor)
            .build()
            .fuse();
        let drain = Async::new(drain).build_with_guard();
        let guard = drain.1;
        let drain = drain.0.fuse();
        Log {
            guard,
            logger: Logger::root(drain, o!())
        }    
    }
}

// source_log macro to print source code information with log messages
macro_rules! source_log {
    ($msg:expr) => {
        concat!(file!(), ":", line!(), ":", column!(), ": ", $msg);
    }
}
