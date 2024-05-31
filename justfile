set dotenv-load := true

default: init

# lists available tasks
@list:
    just --list

# init project
init:
    # bundle install

# start the server
dev:
    # rails server

# open the project in the browser
open:
    # open "$PROJECT_URL" -a "Google Chrome Canary"

# start a console
console:
    # rails console

# run tests
test:
    # rspec

# Open the DB
db:
    litecli db/diesel_demo.db
