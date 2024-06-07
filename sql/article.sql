CREATE TABLE article (
    id              BIGSERIAL PRIMARY KEY,
    kind            VARCHAR(255) NOT NULL,
    tags            VARCHAR(255)[],
    is_trending     BOOLEAN DEFAULT FALSE,
    is_insight      BOOLEAN DEFAULT FALSE,
    is_recommend    BOOLEAN DEFAULT FALSE,
    cover_url       TEXT,
    title           VARCHAR(255) NOT NULL,
    author_id       BIGINT NOT NULL,
    summary         TEXT,
    text            TEXT,
    text_url        TEXT,
    created_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE article_category (
    id          SERIAL PRIMARY KEY,
    level1      VARCHAR(255) NOT NULL,
    level2      VARCHAR(255) NOT NULL,
    level3      VARCHAR(255) NOT NULL,
    description TEXT
);

-- 插入三级分类 (Coding -> Languages -> Each Language)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'Languages', 'C', 'Description for C'),
('Coding', 'Languages', 'C++', 'Description for C++'),
('Coding', 'Languages', 'Python', 'Description for Python'),
('Coding', 'Languages', 'Rust', 'Description for Rust'),
('Coding', 'Languages', 'Golang', 'Description for Golang'),
('Coding', 'Languages', 'Java', 'Description for Java'),
('Coding', 'Languages', 'JavaScript', 'Description for JavaScript');

-- 插入三级分类 (Coding -> Database -> Each DB)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'Database', 'MySQL', 'Description for MySQL'),
('Coding', 'Database', 'PostgreSQL', 'Description for PostgreSQL'),
('Coding', 'Database', 'MongoDB', 'Description for MongoDB'),
('Coding', 'Database', 'SQLite', 'Description for SQLite');

-- 插入三级分类 (Coding -> Cache DB -> Each Cache DB)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'Cache DB', 'Redis', 'Description for Redis'),
('Coding', 'Cache DB', 'Memcached', 'Description for Memcached');

-- 插入三级分类 (Coding -> Frameworks -> Each Framework)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'Frameworks', 'Django', 'Description for Django'),
('Coding', 'Frameworks', 'Flask', 'Description for Flask'),
('Coding', 'Frameworks', 'Vue.js', 'Description for Vue.js'),
('Coding', 'Frameworks', 'Gin', 'Description for Gin'),
('Coding', 'Frameworks', 'Actix-web', 'Description for Actix-web');

-- 插入三级分类 (Coding -> DevOps -> Each Tool)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'DevOps', 'Docker', 'Description for Docker'),
('Coding', 'DevOps', 'Kubernetes', 'Description for Kubernetes'),
('Coding', 'DevOps', 'CI/CD', 'Description for CI/CD'),
('Coding', 'DevOps', 'Monitoring', 'Description for Monitoring');

-- 插入三级分类 (Coding -> Version Control -> Each System)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Coding', 'Version Control', 'Git', 'Description for Git'),
('Coding', 'Version Control', 'SVN', 'Description for SVN');

-- 插入三级分类 (CS -> Theory -> Each Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('CS', 'Theory', 'Algorithms', 'Description for Algorithms'),
('CS', 'Theory', 'Data Structures', 'Description for Data Structures'),
('CS', 'Theory', 'Computability', 'Description for Computability');

-- 插入三级分类 (CS -> Systems -> Each System)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('CS', 'Systems', 'Operating Systems', 'Description for Operating Systems'),
('CS', 'Systems', 'Distributed Systems', 'Description for Distributed Systems'),
('CS', 'Systems', 'Realtime Systems', 'Description for Realtime Systems');

-- 插入三级分类 (CS -> Security -> Each Security Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('CS', 'Security', 'Cryptography', 'Description for Cryptography'),
('CS', 'Security', 'Network Security', 'Description for Network Security'),
('CS', 'Security', 'Web Security', 'Description for Web Security');

-- 插入三级分类 (CS -> Artificial Intelligence -> Each AI Field)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('CS', 'Artificial Intelligence', 'Machine Learning', 'Description for Machine Learning'),
('CS', 'Artificial Intelligence', 'Deep Learning', 'Description for Deep Learning'),
('CS', 'Artificial Intelligence', 'Natural Language Processing', 'Description for Natural Language Processing');


-- 插入三级分类 (Maths -> Fundamentals -> Each Fundamental Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Maths', 'Fundamentals', 'Algebra', 'Description for Algebra'),
('Maths', 'Fundamentals', 'Geometry', 'Description for Geometry'),
('Maths', 'Fundamentals', 'Trigonometry', 'Description for Trigonometry');

-- 插入三级分类 (Maths -> Advanced Mathematics -> Each Advanced Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Maths', 'Advanced Mathematics', 'Calculus', 'Description for Calculus'),
('Maths', 'Advanced Mathematics', 'Linear Algebra', 'Description for Linear Algebra'),
('Maths', 'Advanced Mathematics', 'Differential Equations', 'Description for Differential Equations');

-- 插入三级分类 (Maths -> Statistics and Probability -> Each Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Maths', 'Statistics and Probability', 'Probability', 'Description for Probability'),
('Maths', 'Statistics and Probability', 'Statistics', 'Description for Statistics'),
('Maths', 'Statistics and Probability', 'Bayesian Statistics', 'Description for Bayesian Statistics');

-- 插入三级分类 (Maths -> Discrete Mathematics -> Each Topic)
INSERT INTO article_category (level1, level2, level3, description) VALUES
('Maths', 'Discrete Mathematics', 'Graph Theory', 'Description for Graph Theory'),
('Maths', 'Discrete Mathematics', 'Combinatorics', 'Description for Combinatorics'),
('Maths', 'Discrete Mathematics', 'Number Theory', 'Description for Number Theory');
