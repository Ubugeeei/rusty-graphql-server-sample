run:
	cargo run &

stop:
	lsof -i:8080 -t | xargs kill

test:
	curl -X POST -H "Content-Type: application/json" -d '{"query": "query { staticValue }"}' http://127.0.0.1:8080 | jq
	curl -X POST -H "Content-Type: application/json" -d '{"query": "mutation {createTodo(title: \"aaa\", description: \"\")}"}' http://127.0.0.1:8080 | jq