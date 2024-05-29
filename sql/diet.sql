CREATE TABLE diet (
    id              BIGSERIAL PRIMARY KEY,
    milk            INT NOT NULL,
    meat            INT NOT NULL,
    egg             INT NOT NULL,
    vegetable       INT NOT NULL,
    fruit           INT NOT NULL,
    grain           INT NOT NULL,
    record_date     DATE NOT NULL,
    created_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
)

COMMENT ON COLUMN diet.milk IS 'Unit: milliliters (ml)';
COMMENT ON COLUMN diet.meat IS 'Unit: grams (g)';
COMMENT ON COLUMN diet.egg IS 'Unit: units';
COMMENT ON COLUMN diet.vegetable IS 'Unit: grams (g)';
COMMENT ON COLUMN diet.fruit IS 'Unit: grams (g)';
COMMENT ON COLUMN diet.grain IS 'Unit: grams (g)';