CREATE TABLE projects_techs (
    project_id INTEGER NOT NULL,
    tech_id INTEGER NOT NULL,
    PRIMARY KEY (project_id, tech_id),
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (tech_id) REFERENCES techs(id)
);
