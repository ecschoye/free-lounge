# Makefile for managing Docker Compose

# Variables
DC=docker-compose

# Targets

# Build the services defined in docker-compose.yml
build:
	$(DC) build

# Start the services defined in docker-compose.yml
up:
	$(DC) up -d

# Stop the services defined in docker-compose.yml
down:
	$(DC) down

# Show the logs of services defined in docker-compose.yml
logs:
	$(DC) logs

# Rebuild and restart services
rebuild: down build up

# View currently running containers
ps:
	$(DC) ps

# Connect to the freelounge_backend service for debugging
backend-shell:
	$(DC) exec free_lounge_backend sh

# Connect to the freelounge_frontend service for debugging
frontend-shell:
	$(DC) exec freelounge_frontend sh

# Stop and remove all containers, networks, volumes, and images
clean:
	$(DC) down --volumes --remove-orphans

.PHONY: build up down logs rebuild ps backend-shell frontend-shell clean
