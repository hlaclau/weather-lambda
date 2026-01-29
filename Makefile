.PHONY: build init plan deploy destroy test fmt lint check

build:
	mise exec -- cargo lambda build --release --arm64

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test

init:
	mise exec -- terraform -chdir=terraform init

plan: build
	mise exec -- terraform -chdir=terraform plan

deploy: build
	mise exec -- terraform -chdir=terraform apply

destroy:
	mise exec -- terraform -chdir=terraform destroy
