
default:
    just -l

benchmark:
    @echo "Benchmarking..."
    cargo bench
    @echo "Benchmarking done."

results:
    open http://localhost:8080/report/index.html
    python -m http.server 8080 -d target/criterion
