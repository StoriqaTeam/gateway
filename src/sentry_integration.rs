use failure::Error;
use sentry;
use sentry::integrations::failure::capture_error;

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
    pub dsn: String,
    pub environment: String,
}

pub fn init(sentry_config: Option<&SentryConfig>) -> Option<sentry::internals::ClientInitGuard> {
    sentry_config.map(|config_sentry| {
        println!("initialization support with sentry");
        let result = sentry::init((
            config_sentry.dsn.clone(),
            sentry::ClientOptions {
                release: sentry_crate_release!(),
                environment: Some(config_sentry.environment.clone().into()),
                ..Default::default()
            },
        ));
        sentry::integrations::panic::register_panic_handler();
        result
    })
}

pub fn log_and_capture_error(error: &Error) {
    error!("Internal server error: {:?}", error);
    capture_error(error);
}
