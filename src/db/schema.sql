pragma journal_mode = wal;

create table if not exists viewings (
    id integer primary key,
    movie_id integer references movies (id),
    episode_id integer references episodes (id),
    viewing_date text not null check (date(viewing_date) is not null),
    order_in_day integer not null check (order_in_day >= 0),
    rating real not null check (rating >= 0 and rating <= 10),
    check ((movie_id is null and episode_id is not null) or (movie_id is not null and episode_id is null)),
    unique (viewing_date, order_in_day)
);

create table if not exists tags (
    id integer primary key,
    name text not null unique
);

create table if not exists viewing_tags (
    viewing_id integer not null references viewings (id),
    tag_id integer not null references tags (id),
    primary key (viewing_id, tag_id)
);

create table if not exists movies (
    id integer primary key,
    imdb_id text not null unique,
    original_title text not null,
    title text not null,
    language text not null check (length(language) = 2),
    runtime integer not null check (runtime >= 0),
    release_date text check (release_date is null or date(release_date) is not null),
    overview text
);

create table if not exists series (
    id integer primary key,
    imdb_id text not null unique,
    original_name text not null,
    name text not null,
    language text not null check (length(language) = 2),
    number_of_seasons integer not null check (number_of_seasons >= 0),
    number_of_episodes integer not null check (number_of_episodes >= 0),
    first_air_date text check (first_air_date is null or date(first_air_date) is not null),
    last_air_date text check (last_air_date is null or date(last_air_date) is not null),
    in_production int not null check (in_production in (0, 1)),
    overview text
);

create table if not exists seasons (
    id integer primary key,
    series_id integer not null references series (id),
    number integer not null check (number >= 0),
    name text not null,
    air_date text check (air_date is null or date(air_date) is not null),
    number_of_episodes integer not null check (number_of_episodes >= 0),
    overview text,
    unique (series_id, number)
);

create table if not exists episodes (
    id integer primary key,
    imdb_id text not null unique,
    season_id integer not null references seasons (id),
    number integer not null check (number > 0),
    name text not null,
    air_date text check (air_date is null or date(air_date) is not null),
    runtime integer not null check (runtime >= 0),
    overview text,
    unique (season_id, number)
);

create table if not exists people (
    id integer primary key,
    name text not null,
    gender text not null check (gender in ('U', 'M', 'F', 'N'))
);

create table if not exists cast (
    id integer primary key,
    movie_id integer references movies (id),
    episode_id integer references episodes (id),
    person_id integer not null references people (id),
    character text not null,
    credit_order integer not null check (credit_order >= 0),
    check ((movie_id is null and episode_id is not null) or (movie_id is not null and episode_id is null)),
    unique (movie_id, episode_id, credit_order)
);

create table if not exists crew (
    id integer primary key,
    movie_id integer references movies (id),
    episode_id integer references episodes (id),
    person_id integer not null references people (id),
    job text not null check (job in ('Producer', 'Director', 'Writer')),
    check ((movie_id is null and episode_id is not null) or (movie_id is not null and episode_id is null)),
    unique (movie_id, episode_id, person_id, job)
);

create table if not exists series_creators (
    series_id integer not null references series (id),
    person_id integer not null references people (id),
    primary key (series_id, person_id)
)