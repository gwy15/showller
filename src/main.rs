use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use icalendar::Calendar;
use tracing::Instrument;
use tv_calendar::{calendar::CalendarExt, config, models, tmdb};

async fn get_show(
    show_id: u64,
    client: tmdb::Client,
) -> Result<(models::Show, Vec<models::Season>)> {
    let show = client.get_show(show_id).await.context("get show failed")?;

    let futures: Vec<_> = show
        .seasons
        .iter()
        .map(|s| {
            client
                .get_season(show.id, s.season_number)
                .instrument(tracing::info_span!(
                    "get season",
                    season_number = s.season_number
                ))
        })
        .collect();
    let seasons = futures::future::try_join_all(futures)
        .await
        .with_context(|| format!("fetch seasons from TMDB failed for season {}", show.name))?;

    Ok((show, seasons))
}

async fn generate_calendar(
    user_follows: &[config::UserShow],
    client: &tmdb::Client,
) -> Result<Calendar> {
    let mut calendar = icalendar::Calendar::new()
        .name("tv-calendar")
        .description("Show air dates from TMDB")
        .ttl(&chrono::Duration::seconds(30))
        .done();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    for user_show in user_follows {
        let tx = tx.clone();
        let client = client.clone();
        let show_id = user_show.id;
        let user_show = user_show.clone();

        let fut = async move {
            let result = get_show(show_id, client)
                .await
                .context("get show failed")
                .map(|(show, seasons)| (show, seasons, user_show));
            tx.send(result)
                .await
                .context("send show to calendar failed")?;
            anyhow::Ok(())
        };
        tokio::spawn(fut.instrument(tracing::debug_span!("get_show")));
    }
    std::mem::drop(tx);

    while let Some(result) = rx.recv().await {
        let (show, seasons, user_show) = result?;
        calendar.insert_show_seasons(&show, &seasons, user_show);
        tracing::debug!(show_name=%show.name, "show done");
    }

    Ok(calendar)
}

async fn calendar(
    config: web::Data<config::Config>,
    client: web::Data<tmdb::Client>,
) -> impl Responder {
    tracing::info!("getting calendar");
    let user_shows = &config.show;

    let calendar = match generate_calendar(user_shows, &client)
        .await
        .context("generate calendar for user failed")
    {
        Ok(calendar) => calendar,
        Err(e) => {
            tracing::error!("{:?}", e);
            return HttpResponse::InternalServerError().body("internal server error");
        }
    };

    HttpResponse::Ok().body(calendar.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = match config::Config::load("./config.toml").await {
        Ok(c) => c,
        Err(e) => {
            tracing::info!(?e, "load config file failed, try /config/config.toml");
            config::Config::load("/config/config.toml").await?
        }
    };

    let config = web::Data::new(config);
    let server_config = config.server.clone();
    let client = web::Data::new(config.tmdb.client()?);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(client.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/calendar", web::get().to(calendar))
    });

    for tcp in server_config.tcp.iter() {
        server = server.bind(tcp)?;
    }
    #[cfg(unix)]
    for unix in server_config.uds.iter() {
        server = server.bind_uds(unix)?;
    }
    server.run().await?;

    Ok(())
}
