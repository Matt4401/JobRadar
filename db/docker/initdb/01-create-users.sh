#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

render() {
    local content
    content="$(cat "$1")"
    content="${content//'${JOBRADAR_APP_USER}'/$JOBRADAR_APP_USER}"
    content="${content//'${JOBRADAR_APP_PASSWORD}'/$JOBRADAR_APP_PASSWORD}"
    content="${content//'${JOBRADAR_MIGRATOR_USER}'/$JOBRADAR_MIGRATOR_USER}"
    content="${content//'${JOBRADAR_MIGRATOR_PASSWORD}'/$JOBRADAR_MIGRATOR_PASSWORD}"
    content="${content//'${MYSQL_DATABASE}'/$MYSQL_DATABASE}"
    printf '%s\n' "$content"
}

run_sql_template() {
    render "$1" | mysql --protocol=socket -uroot -p"${MYSQL_ROOT_PASSWORD}"
}

run_sql_template "$SCRIPT_DIR/sql/01-app-user.sql"
run_sql_template "$SCRIPT_DIR/sql/02-migrator-user.sql"
