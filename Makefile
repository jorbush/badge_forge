.PHONY: docker run clean

docker:
	docker build -t badge_forge .
	docker run -d --name badge_forge -p 4000:4000 badge_forge

run:
	cargo run

clean:
	-docker stop badge_forge
	-docker rm badge_forge
