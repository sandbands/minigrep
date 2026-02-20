alias minigrep_test="cargo bench && cargo check && cargo test && cargo clean;";
alias minigrep_deploy="git add . && git push && git commit;";

minigrep_test
minigrep_deploy
