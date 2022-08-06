use icalendar::Component;

use crate::models;

fn s(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub trait CalendarExt {
    fn insert_show_seasons(&mut self, show: &models::Show, seasons: &[models::Season]);
}

impl CalendarExt for icalendar::Calendar {
    fn insert_show_seasons(&mut self, show: &models::Show, seasons: &[models::Season]) {
        for season in seasons {
            for episode in season.episodes.iter() {
                let ep_overview = s(&episode.overview)
                    .or_else(|| s(&show.overview))
                    .unwrap_or("没有简介");
                let ep_name = &episode.name;
                let url = format!(
                    "https://www.themoviedb.org/tv/{}/season/{}/episode/{}",
                    show.id, season.season_number, episode.episode_number
                );

                let event = icalendar::Event::new()
                    .summary(&format!(
                        "{} - S{:02}E{02} - {ep_name}",
                        show.name, season.season_number, episode.episode_number
                    ))
                    .description(&format!("{}\n\n{}", ep_name, ep_overview))
                    .url(&url)
                    .all_day(episode.air_date)
                    .done();
                self.push(event);
            }
        }
    }
}
