CREATE TABLE IF NOT EXISTS resumes (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    pdf TEXT NOT NULL,
    latex_source TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS applications (
    id INTEGER PRIMARY KEY,
    company TEXT NOT NULL,
    position TEXT NOT NULL,
    website TEXT,
    status TEXT DEFAULT 'applied',
    resume INTEGER REFERENCES resumes(id),
    notes TEXT,
    refer TEXT,
    apply_date TEXT NOT NULL
);
