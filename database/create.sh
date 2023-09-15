echo "" > database.sqlite 
sqlite3 database.sqlite <<'END_SQL'
.timeout 2000

CREATE TABLE IF NOT EXISTS project (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
   	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS list (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   id_project INTEGER NOT NULL,
   UNIQUE (id_project, name),
   FOREIGN KEY (id_project) 
   REFERENCES project (id) 
      ON DELETE CASCADE 
      ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS user (
	email TEXT PRIMARY KEY,
   name TEXT NOT NULL,
   password TEXT NOT NULL,
   UNIQUE (email)
);


CREATE TABLE IF NOT EXISTS task (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   tag INTEGER DEFAULT 0,
   priority INTEGER DEFAULT 0,
   state INTEGER DEFAULT 0 NOT NULL,
   descr TEXT DEFAULT "" NOT NULL,
   id_last_editor INTEGER,
   id_list INTEGER NOT NULL,
   FOREIGN KEY (id_last_editor) 
   REFERENCES user (email) 
      ON DELETE CASCADE 
      ON UPDATE NO ACTION,
   FOREIGN KEY (id_list) 
   REFERENCES list (id) 
      ON DELETE CASCADE 
      ON UPDATE NO ACTION
);

CREATE TABLE connexion(
   token TEXT PRIMARY KEY,
   email TEXT,
   FOREIGN KEY (email) 
      REFERENCES user (email) 
         ON DELETE CASCADE 
         ON UPDATE NO ACTION
);

-- indexed table


CREATE TABLE project_user(
   id_project INTEGER,
   email TEXT,
   right INTEGER DEFAULT 0,
   PRIMARY KEY (id_project, email),
   FOREIGN KEY (id_project) 
      REFERENCES project (id) 
         ON DELETE CASCADE 
         ON UPDATE NO ACTION,
   FOREIGN KEY (email) 
      REFERENCES user (email) 
         ON DELETE CASCADE 
         ON UPDATE NO ACTION
);



END_SQL