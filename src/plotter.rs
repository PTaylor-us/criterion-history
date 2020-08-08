use plotters::prelude::*;

use chrono::{Datelike, TimeZone, Timelike, Utc};

use crate::Log;
use std::error::Error;

pub(crate) fn plot(log: &Log) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new("plot.svg", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let first_timestamp = log
        .values()
        .map(|x| x.iter().min_by_key(|a| a.timestamp).unwrap().timestamp)
        .min()
        .unwrap();
    let last_timestamp = log
        .values()
        .map(|x| x.iter().max_by_key(|a| a.timestamp).unwrap().timestamp)
        .max()
        .unwrap();
    let first_timestamp = Utc
        .datetime_from_str(first_timestamp.to_string().as_str(), "%Y%m%d%H%M")
        .unwrap();
    let last_timestamp = Utc
        .datetime_from_str(last_timestamp.to_string().as_str(), "%Y%m%d%H%M")
        .unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Average Duration of One Iteration", ("sans-serif", 40))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(first_timestamp..last_timestamp, 0.0..200.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(10)
        .x_label_formatter(&|d| {
            format!(
                "{}-{} {:02}:{:02}",
                d.month(),
                d.day(),
                d.hour(),
                d.minute()
            )
        })
        .y_desc("nanoseconds")
        .draw()?;

    for (count, test) in (0..).zip(log) {
        chart
            .draw_series(LineSeries::new(
                test.1.iter().map(|log_data| {
                    (
                        Utc.datetime_from_str(
                            log_data.timestamp.to_string().as_str(),
                            "%Y%m%d%H%M",
                        )
                        .unwrap(),
                        log_data.average_duration * 1_000_000_000.0,
                    )
                }),
                &Palette99::pick(count),
            ))?
            .label(test.0)
            .legend(move |(x, y)| Circle::new((x, y), 4, Palette99::pick(count).filled()));

        chart.draw_series(test.1.iter().map(|log_data| {
            Circle::new(
                (
                    Utc.datetime_from_str(log_data.timestamp.to_string().as_str(), "%Y%m%d%H%M")
                        .unwrap(),
                    log_data.average_duration * 1_000_000_000.0,
                ),
                3,
                Palette99::pick(count).filled(),
            )
        }))?;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
