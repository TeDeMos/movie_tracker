pragma journal_mode = wal;

create table if not exists movies (
    id integer primary key,
    imdb_id text not null,
    language text not null,
    title text not null,
    overview text,
    release_date text check (release_date is null or date(release_date) is not null),
    runtime integer not null check(runtime >= 0)
);

create table if not exists people (
    id integer primary key,
    name text not null
);

create table if not exists cast (
    movie_id integer not null references movies (id),
    person_id integer not null references people (id),
    character text not null,
    credit_order integer not null check(credit_order >= 0),
    primary key (movie_id, person_id)
);

create table if not exists crew (
    movie_id integer not null references movies (id),
    person_id integer not null references people (id),
    job text not null check (job in ('Producer', 'Director', 'Writer')),
    primary key (movie_id, person_id, job)
);