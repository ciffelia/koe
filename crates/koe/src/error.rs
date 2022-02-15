use log::error;
use sentry::integrations::anyhow::capture_anyhow;

pub fn report_error(err: impl Into<anyhow::Error>) {
    let err = err.into();

    error!("{:?}", err);
    capture_anyhow(&err);
}
