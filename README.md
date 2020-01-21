# actix-request-body-backpressure

```sh
cargo run

truncate -s 50M 50M.bin
curl -v http://127.0.0.1:8080/ -X PUT --data-binary @50M.bin > /dev/null

ps -o rss=,command ax | grep actix-request-body-backpressure | grep -v grep
```
