/* Logger initialization */
use std::{panic, thread};

use tracing::{error, level_filters::LevelFilter};
use tracing_appender::non_blocking::WorkerGuard;

use crate::CargoEnv;

pub struct LoggerGuards {
    pub _tracing_guard: WorkerGuard,
    pub _sentry_guard: Option<sentry::ClientInitGuard>,
}

pub struct Logger {}

impl Logger {
    pub fn init(cargo_env: CargoEnv, sentry_dsn: Option<String>) -> LoggerGuards {
        let file_logger = tracing_appender::rolling::daily("logs", "daily.log");
        let console_logger = std::io::stdout();

        // dev is honestly nice even in prod but the option is here
        //
        // i like info for both but debug is better if you don't want the noise
        let max_level = match cargo_env {
            CargoEnv::Development => LevelFilter::INFO,
            CargoEnv::Production => LevelFilter::INFO,
        };

        // again here, most cdns captures stdout, but if needed, logging to file is an
        // option
        let (non_blocking, guard) = match cargo_env {
            CargoEnv::Development => tracing_appender::non_blocking(console_logger),
            CargoEnv::Production => tracing_appender::non_blocking(file_logger),
        };

        let sentry_guard = sentry_dsn.map(|dsn| {
            sentry::init((
                dsn,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    environment: Some(match cargo_env {
                        CargoEnv::Development => "development".into(),
                        CargoEnv::Production => "production".into(),
                    }),
                    attach_stacktrace: true,
                    ..Default::default()
                },
            ))
        });

        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;

        let fmt_layer = tracing_subscriber::fmt::layer().with_writer(non_blocking);

        let registry = tracing_subscriber::registry()
            .with(max_level)
            .with(fmt_layer);

        if sentry_guard.is_some() {
            registry.with(sentry_tracing::layer()).init();
        } else {
            registry.init();
        }

        // oopsie bad
        panic::set_hook(Box::new(|info| {
            let thread = thread::current();
            let thread = thread.name().unwrap_or("unknown");

            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                // maybe it's on the heap!
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    // what the fuck
                    None => "Box<Any>",
                },
            };

            let backtrace = backtrace::Backtrace::new();

            match info.location() {
                Some(location) => {
                    // we have no trace so just do the weird panic
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}",
                            thread,
                            msg.replace("notrace - ", ""),
                            location.file(),
                            location.line()
                        );
                    }
                    // we have a trace so we do full panic
                    else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                            thread,
                            msg,
                            location.file(),
                            location.line(),
                            backtrace
                        );
                    }
                }
                // what even happens to get here
                None => {
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'",
                            thread,
                            msg.replace("notrace - ", ""),
                        );
                    } else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                            thread,
                            msg,
                            backtrace
                        );
                    }
                }
            }
        }));

        // return both guards so they're not dropped
        LoggerGuards {
            _tracing_guard: guard,
            _sentry_guard: sentry_guard,
        }
    }
}
