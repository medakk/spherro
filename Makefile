clean:
	rm -rf _build
	-docker rm spherrobuildcontainer

build: clean
	docker build -t spherrobuild .
	docker create --name spherrobuildcontainer spherrobuild
	docker cp spherrobuildcontainer:/build/www/dist/ _build
