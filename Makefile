.PHONY: build init plan deploy destroy

build:
	mise exec -- cargo lambda build --release --arm64

init:
	mise exec -- terraform -chdir=terraform init

plan: build
	mise exec -- terraform -chdir=terraform plan

deploy: build
	mise exec -- terraform -chdir=terraform apply

destroy:
	mise exec -- terraform -chdir=terraform destroy
