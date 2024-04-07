build-no-cache:
	docker-compose build --no-cache
	docker-compose up -d --remove-orphans

build:
	docker-compose build
	docker-compose up -d --remove-orphans

start:
	docker-compose up -d

pause:
	docker-compose stop

destroy:
	docker-compose down --remove-orphans

destroy-and-clean:
	docker-compose down -v --remove-orphans
	docker system prune -a -f

rebuild:
	docker-compose down -v --remove-orphans
	docker-compose up -d
