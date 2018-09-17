-- Schema for psql food database

CREATE TABLE IF NOT EXISTS categories (
	category_id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ingredients (
	ingredient_id	SERIAL	PRIMARY KEY,
	name	TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units (
	unit_id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS containers (
	container_id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tools (
	tool_id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe (
	recipe_id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	category_id INTEGER REFERENCES categories(category_id)
);

CREATE TABLE IF NOT EXISTS verbs (
	verb_id SERIAL PRIMARY KEY,
	verb TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS verb_objects (
	verb_object_id SERIAL PRIMARY KEY,
	verb_id INTEGER REFERENCES verbs(verb_id),
	ingredient_id INTEGER REFERENCES ingredients(ingredient_id),

	amount REAL NOT NULL,
	unit_id INTEGER REFERENCES units(unit_id)
);

CREATE TABLE IF NOT EXISTS instructions (
	instruction_id SERIAL	PRIMARY KEY,
	recipe_id INTEGER REFERENCES recipe(recipe_id)
);

CREATE TABLE IF NOT EXISTS steps (
	step_id SERIAL	PRIMARY KEY,
	instruction_id INTEGER REFERENCES instructions(instruction_id),
	verb_id INTEGER REFERENCES verbs(verb_id),
	stop_when TEXT,
	stop_after REAL, --in seconds,
	container_id INTEGER REFERENCES containers(container_id),
	tool_id INTEGER REFERENCES tools(tool_id)
);
