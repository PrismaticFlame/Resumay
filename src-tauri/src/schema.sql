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
    username TEXT,
    status TEXT DEFAULT 'applied',
    resume INTEGER REFERENCES resumes(id),
    notes TEXT,
    refer TEXT,
    apply_date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY,
    section TEXT NOT NULL,
    title TEXT NOT NULL,
    subtitle TEXT,
    dates TEXT,
    sort_order INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS bullets (
    id INTEGER PRIMARY KEY,
    entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    sort_order INTEGER DEFAULT 0
);