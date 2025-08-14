use chrono::{Local};
use owo_colors::{OwoColorize, Style};
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{self, format::FmtSpan, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use serde::{Deserialize, Deserializer};
use serde::de::value::StringDeserializer;

pub(crate) struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
        let meta = event.metadata();

        let level = get_level_style(*meta.level());

        write!(writer, "{}  {}  ::  ", now.bright_black(), level)?;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn get_level_style(level: tracing::Level) -> impl std::fmt::Display {
    let style = match level {
        tracing::Level::ERROR => Style::new().red().bold(),
        tracing::Level::WARN => Style::new().yellow().bold(),
        tracing::Level::INFO => Style::new().green().bold(),
        tracing::Level::DEBUG => Style::new().blue().bold(),
        tracing::Level::TRACE => Style::new().magenta().bold(),
    };
    style.style(level)
}


pub fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));


    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .event_format(CustomFormatter)
        .init();
}

pub fn deserialize_csv_statuses<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Input<T> {
        Csv(String),
        Seq(Vec<T>),
    }

    let input = Option::<Input<T>>::deserialize(deserializer)?;
    Ok(match input {
        None => None,
        Some(Input::Seq(v)) => Some(v),
        Some(Input::Csv(s)) => {
            if s.trim().is_empty() {
                Some(Vec::new())
            } else {
                Some(
                    s.split(',')
                        .map(|part| {
                            let part = part.trim();
                            T::deserialize(StringDeserializer::<serde::de::value::Error>::new(
                                part.to_owned(),
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(serde::de::Error::custom)?,
                )
            }
        }
    })
}

