# Variables
CARGO = cargo
PSQL = psql
DB_USER = sjulliot
DB_NAME = runners

all: build

build:
	$(CARGO) build

run:
	$(CARGO) run

test:
	$(CARGO) test

clean:
	$(CARGO) clean

fmt:
	$(CARGO) fmt

check:
	$(CARGO) check

psql:
	$(PSQL) -U $(DB_USER) -d $(DB_NAME)

help:
	@echo "Makefile commands:"
	@echo "  all      - Build the project"
	@echo "  build    - Build the project"
	@echo "  run      - Run the project"
	@echo "  test     - Test the project"
	@echo "  clean    - Clean the project"
	@echo "  fmt      - Format the code"
	@echo "  check    - Check the code"
	@echo "  psql     - Run psql command"
	@echo "  help     - Show this help message"

.PHONY: all build run test clean fmt check psql help