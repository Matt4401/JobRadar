### The goal of this script is to create two non-root users for the application.

This script is executed automatically by the MySQL image because it lives in
/docker-entrypoint-initdb.d/. At that point the database named by
MYSQL_DATABASE already exists (the image creates it), so this only needs to
create the users and grant them privileges scoped to that single database.

Principle of least privilege:
   - **app user**:
     > DML only (SELECT/INSERT/UPDATE/DELETE): used by the API and the scraper at runtime.
   - **migrator user**: 
     > DML + DDL (CREATE/ALTER/DROP/INDEX...): used ONLY by the migration CLI to apply schema changes.

*NOTE: init scripts run only when the data volume is empty. To re-run them on
an already-initialized database, recreate the volume: `docker compose down -v`.*
