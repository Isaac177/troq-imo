.PHONY: dev dev-infra

dev: dev-infra
	@set -a; . ./infra/.env.dev; set +a; cd backend; exec cargo watch -x 'run -p api --bin mungini-api'

dev-infra:
	@test -f infra/.env.dev || cp infra/.env.example infra/.env.dev
	docker compose --env-file infra/.env.dev -f infra/docker-compose.dev.yml up -d --wait db redis minio
	docker compose --env-file infra/.env.dev -f infra/docker-compose.dev.yml run --rm --no-deps minio-init
