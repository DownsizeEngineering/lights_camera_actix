CREATE SCHEMA done;

CREATE TABLE done.lists (
    id SERIAL NOT NULL,
    name VARCHAR(100),
    PRIMARY KEY (id) 
);

CREATE TABLE done.todos (
    id SERIAL NOT NULL,
    task VARCHAR(100),
    details VARCHAR(255),
    completed BOOLEAN NOT NULL,
    list INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY (list) REFERENCES done.lists (id)
);