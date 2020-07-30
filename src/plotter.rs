use plotters::coord::IntoMonthly;
use plotters::prelude::*;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};

use crate::Log;
use std::error::Error;

pub(crate) fn plot(log: &Log) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new("plot.svg", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(
            "Monthly Average Temperate in Salt Lake City, UT",
            ("sans-serif", 40),
        )
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(
            (Utc.ymd(2020, 7, 29).and_hms(0, 0, 0)..Utc.ymd(2020, 7, 31).and_hms(0, 0, 0)),
            0.0..200.0,
        )?;

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

const DATA: [(i32, u32, f64); 12 * 9] = [
    (2010, 1, 32.4),
    (2010, 2, 37.5),
    (2010, 3, 44.5),
    (2010, 4, 50.3),
    (2010, 5, 55.0),
    (2010, 6, 70.0),
    (2010, 7, 78.7),
    (2010, 8, 76.5),
    (2010, 9, 68.9),
    (2010, 10, 56.3),
    (2010, 11, 40.3),
    (2010, 12, 36.5),
    (2011, 1, 28.8),
    (2011, 2, 35.1),
    (2011, 3, 45.5),
    (2011, 4, 48.9),
    (2011, 5, 55.1),
    (2011, 6, 68.8),
    (2011, 7, 77.9),
    (2011, 8, 78.4),
    (2011, 9, 68.2),
    (2011, 10, 55.0),
    (2011, 11, 41.5),
    (2011, 12, 31.0),
    (2012, 1, 35.6),
    (2012, 2, 38.1),
    (2012, 3, 49.1),
    (2012, 4, 56.1),
    (2012, 5, 63.4),
    (2012, 6, 73.0),
    (2012, 7, 79.0),
    (2012, 8, 79.0),
    (2012, 9, 68.8),
    (2012, 10, 54.9),
    (2012, 11, 45.2),
    (2012, 12, 34.9),
    (2013, 1, 19.7),
    (2013, 2, 31.1),
    (2013, 3, 46.2),
    (2013, 4, 49.8),
    (2013, 5, 61.3),
    (2013, 6, 73.3),
    (2013, 7, 80.3),
    (2013, 8, 77.2),
    (2013, 9, 68.3),
    (2013, 10, 52.0),
    (2013, 11, 43.2),
    (2013, 12, 25.7),
    (2014, 1, 31.5),
    (2014, 2, 39.3),
    (2014, 3, 46.4),
    (2014, 4, 52.5),
    (2014, 5, 63.0),
    (2014, 6, 71.3),
    (2014, 7, 81.0),
    (2014, 8, 75.3),
    (2014, 9, 70.0),
    (2014, 10, 58.6),
    (2014, 11, 42.1),
    (2014, 12, 38.0),
    (2015, 1, 35.3),
    (2015, 2, 45.2),
    (2015, 3, 50.9),
    (2015, 4, 54.3),
    (2015, 5, 60.5),
    (2015, 6, 77.1),
    (2015, 7, 76.2),
    (2015, 8, 77.3),
    (2015, 9, 70.4),
    (2015, 10, 60.6),
    (2015, 11, 40.9),
    (2015, 12, 32.4),
    (2016, 1, 31.5),
    (2016, 2, 35.1),
    (2016, 3, 49.1),
    (2016, 4, 55.1),
    (2016, 5, 60.9),
    (2016, 6, 76.9),
    (2016, 7, 80.0),
    (2016, 8, 77.0),
    (2016, 9, 67.1),
    (2016, 10, 59.1),
    (2016, 11, 47.4),
    (2016, 12, 31.8),
    (2017, 1, 29.4),
    (2017, 2, 42.4),
    (2017, 3, 51.7),
    (2017, 4, 51.7),
    (2017, 5, 62.5),
    (2017, 6, 74.8),
    (2017, 7, 81.3),
    (2017, 8, 78.1),
    (2017, 9, 65.7),
    (2017, 10, 52.5),
    (2017, 11, 49.0),
    (2017, 12, 34.4),
    (2018, 1, 38.1),
    (2018, 2, 37.5),
    (2018, 3, 45.4),
    (2018, 4, 54.6),
    (2018, 5, 64.0),
    (2018, 6, 74.9),
    (2018, 7, 82.5),
    (2018, 8, 78.1),
    (2018, 9, 71.9),
    (2018, 10, 53.2),
    (2018, 11, 39.7),
    (2018, 12, 33.6),
];
