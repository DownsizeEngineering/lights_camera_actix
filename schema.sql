CREATE SCHEMA done;

CREATE TABLE done.lists (
    id SERIAL NOT NULL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY (id) 
);

CREATE TABLE done.todos (
    id SERIAL NOT NULL,
    task VARCHAR(100) NOT NULL,
    details VARCHAR(255),
    completed BOOLEAN NOT NULL,
    list INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (list) REFERENCES done.lists (id)
);