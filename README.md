I put together a `Wrapper` structure that is responsible for communicating with the endpoint.
This allows the separation of testing logic and API specifics. If an endpoint changes we only have 
to update the corresponding wrapper function for example.

To run the tests, you'll need to provide the endpoint's root URI and a token with administrative
rights:
```
cargo run -- -t Oy3hfPoH45ze7Q -u http://ec2-3-65-182-233.eu-central-1.compute.amazonaws.com:8080
```
Running `cargo run -- -h` prints out a help message.

Logging support offers different levels of verbosity. The default level is `info` which will report 
the results of the tests, whereas `warn` will only report failed tests, and `error` will report
nothing. The return code is `0` if all tests pass and `1` otherwise.
