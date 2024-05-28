CREATE TABLE behavior (
    id              BIGSERIAL PRIMARY KEY,
    wake_up_time    TIME NOT NULL,
    sleep_time      TIME NOT NULL,
    diaper_changes  INT NOT NULL,
    naps            INT NOT NULL,
    crying_episodes INT NOT NULL,
    outdoor_time    INTERVAL NOT NULL,
    record_date     DATE NOT NULL
);
