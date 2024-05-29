CREATE TABLE health (
    id                  BIGSERIAL PRIMARY KEY,
    height              DOUBLE PRECISION NOT NULL,
    weight              DOUBLE PRECISION NOT NULL,
    teeth               INT NOT NULL,
    head_circumference  DOUBLE PRECISION NOT NULL,
    record_date         DATE NOT NULL,
    created_at          TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
)