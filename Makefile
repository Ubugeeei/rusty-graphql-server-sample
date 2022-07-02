run:
	cargo run &

stop:
	lsof -i:8080 -t | xargs kill

test:
	make run &
	sleep 5s
	curl http://127.0.0.1:8080/hello/Ubugeeei
	make stop