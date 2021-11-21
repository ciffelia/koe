use log::error;

pub fn report_error(err: impl Into<anyhow::Error>) {
    let err = err.into();

    error!("{:?}", err);
    sentry_anyhow::capture_anyhow(&err);
}
