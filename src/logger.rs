use {
    anyhow::Result,
    tracing_subscriber::{fmt, prelude::*},
};

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {{
        tracing::warn!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};

    ($fmt:expr, $($args:tt)*) => {
        {
            tracing::warn!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

#[macro_export]
macro_rules! panic {
    ($fmt:expr) => {{
        tracing::error!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};
    ($fmt:expr, $($args:tt)*) => {
        {
            tracing::error!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => {{
        tracing::error!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};
    ($fmt:expr, $($args:tt)*) => {
        {
            tracing::error!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {{
        tracing::debug!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};

    ($fmt:expr, $($args:tt)*) => {
        {
            tracing::debug!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

pub fn init(
    file: Option<String>,
    verbose: bool,
) -> Result<Option<tracing_appender::non_blocking::WorkerGuard>> {
    let subscriber =
        tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::from_default_env());
    let ret = if let Some(file) = file {
        let path = std::path::Path::new(file.as_str());
        let filename = path.file_name().unwrap();
        let dir = file.replace(filename.to_str().unwrap(), "");
        let file_appender = tracing_appender::rolling::daily(dir, filename);
        let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
        let subscriber = subscriber.with(fmt::Layer::new().with_writer(file_writer));
        if !verbose {
            tracing::subscriber::set_global_default(subscriber)?;
        } else {
            let subscriber = subscriber.with(fmt::Layer::new().with_writer(std::io::stdout));
            tracing::subscriber::set_global_default(subscriber)?;
        }
        Some(_guard)
    } else {
        let subscriber = subscriber.with(fmt::Layer::default().with_writer(std::io::stdout));
        tracing::subscriber::set_global_default(subscriber)?;
        None
    };
    Ok(ret)
}
