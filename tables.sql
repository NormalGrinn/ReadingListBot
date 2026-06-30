CREATE TABLE anime (
    media_id INTEGER PRIMARY KEY REFERENCES media(media_id),
    al_id INTEGER UNIQUE,
    title TEXT,
    format TEXT,
    season TEXT,
    seasonYear INTEGER,
    source TEXT,
    synonyms TEXT,
    cover_image_small TEXT
);

CREATE TABLE people (
    person_id INTEGER PRIMARY KEY,
    person_name TEXT,
    person_synopsis TEXT
);

CREATE TABLE resources (
    resource_id INTEGER PRIMARY KEY,
    link TEXT UNIQUE,
    resource_title TEXT NOT NULL,
    resource_synopsis TEXT,
    resource_type TEXT NOT NULL,
    resource_language TEXT,
    author TEXT
);

CREATE TABLE tags (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT UNIQUE
);

CREATE TABLE media (
    media_id  INTEGER PRIMARY KEY,
    media_type TEXT NOT NULL CHECK(media_type IN ('ANIME'))
);

CREATE TABLE resource_people (
    resource_id INTEGER,
    person_id INTEGER,

    PRIMARY KEY (resource_id, person_id),

    FOREIGN KEY (resource_id) REFERENCES resources(resource_id),
    FOREIGN KEY (person_id) REFERENCES people(person_id)
);

CREATE TABLE resource_tags (
    resource_id INTEGER,
    tag_id INTEGER,

    PRIMARY KEY (resource_id, tag_id),

    FOREIGN KEY (resource_id) REFERENCES resources(resource_id),
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
);

CREATE TABLE resource_media (
    resource_id INTEGER REFERENCES resources(resource_id),
    media_id    INTEGER REFERENCES media(media_id),
    PRIMARY KEY (resource_id, media_id)
);