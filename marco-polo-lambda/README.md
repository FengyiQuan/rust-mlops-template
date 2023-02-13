### Steps to run

* `make format` to format code
* `make lint` to lint
* `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
* `make deploy` which is this`cargo lambda deploy`

```Working demo
(.venv) @FengyiQuan ➜ /workspaces/rust-mlops-template/marco-polo-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                marco-polo-lambda
{
  "msg": "Marco says Polo",
  "req_id": "82c90ae1-4bce-4eac-8de2-c8952d2b376d"
}
```
