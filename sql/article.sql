CREATE TABLE article (
    id              BIGSERIAL PRIMARY KEY,
    cover_url       TEXT NOT NULL,
    title           VARCHAR(255) NOT NULL,
    author_id       BIGINT NOT NULL,
    category_id     INT NOT NULL,
    summary         TEXT,
    text            TEXT NOT NULL,
    created_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE article_category (
    id          SERIAL PRIMARY KEY,
    level1      VARCHAR(255) NOT NULL,
    level2      VARCHAR(255) NOT NULL,
    level3      VARCHAR(255) NOT NULL,
    description TEXT
)

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

