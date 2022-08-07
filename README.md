# TV-calendar
[![Continuous integration](https://github.com/gwy15/tv-calendar/actions/workflows/ci.yml/badge.svg)](https://github.com/gwy15/tv-calendar/actions/workflows/ci.yml)
[![Publish Docker image](https://github.com/gwy15/tv-calendar/actions/workflows/docker.yml/badge.svg)](https://github.com/gwy15/tv-calendar/actions/workflows/docker.yml)

A nice tool that generates an iCalendar format file for you to follow your favorite TV shows.

# Usage
copy `examples/config.toml` to `./config.toml` or `/config/config.toml` (preferable for docker), then visit url from 
`http://{host}/calendar`.

## Source of Data
The source of data is [TMDB (The Movie DB)](https://www.themoviedb.org/).

