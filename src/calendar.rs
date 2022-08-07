use icalendar::Component;

use crate::{config, models};

fn s(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub trait CalendarExt {
    fn insert_show_seasons(
        &mut self,
        show: &models::Show,
        seasons: &[models::Season],
        user_show: config::UserShow,
    );
}

impl CalendarExt for icalendar::Calendar {
    fn insert_show_seasons(
        &mut self,
        show: &models::Show,
        seasons: &[models::Season],
        user_show: config::UserShow,
    ) {
        let offset = user_show.offset_days.unwrap_or(0);
        let now = chrono::Utc::now();
        let last_modified = now.format("%Y%m%dT%H%M%SZ").to_string();

        for season in seasons {
            for episode in season.episodes.iter() {
                let ep_name = &episode.name;
                let tmdb_url = format!(
                    "https://www.themoviedb.org/tv/{}/season/{}/episode/{}",
                    show.id, season.season_number, episode.episode_number
                );
                let air_date = episode.air_date + chrono::Duration::days(offset as i64);

                let ep_overview = s(&episode.overview)
                    .or_else(|| s(&show.overview))
                    .unwrap_or("没有简介");
                let url = user_show
                    .url
                    .as_deref()
                    .unwrap_or_default()
                    .replace("{s:02}", &format!("{:02}", season.season_number))
                    .replace("{e:02}", &format!("{:02}", episode.episode_number));
                let description = format!(
                    "\
                    {}\n\
                    \n\
                    TMDB: {}\n\
                    {}\n",
                    ep_overview, tmdb_url, url
                );

                let event = icalendar::Event::new()
                    .summary(&format!(
                        "{} - S{:02}E{02} - {ep_name}",
                        show.name, season.season_number, episode.episode_number
                    ))
                    .description(&description)
                    .add_property("UID", &tmdb_url)
                    .add_property("LAST-MOD", &last_modified)
                    .url(&tmdb_url)
                    .all_day(air_date)
                    .done();
                self.push(event);
            }
        }
    }
}
