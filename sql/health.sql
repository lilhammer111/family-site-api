CREATE TABLE health (
    id                  BIGSERIAL PRIMARY KEY,
    height              DOUBLE PRECISION NOT NULL,
    weight              DOUBLE PRECISION NOT NULL,
    teeth               SMALLINT NOT NULL,
    head_circumference  DOUBLE PRECISION NOT NULL,
    measurement_date    DATE NOT NULL
)