build:
	docker build . -t micro-serv-rust:latest

run-app:
	docker run micro-serv-rust:latest

run-all:
	docker-compose up