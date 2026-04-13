build_analysis:
	wasm-pack build -s palform -d ../analysis-js --out-name analysis --dev packages/analysis

build_client_common:
	wasm-pack build -s palform -d ../client-js --dev packages/client-common --no-default-features --features frontend-js,debug

build_crypto:
	wasm-pack build -s palform -d ../crypto-js --dev packages/crypto --no-default-features --features frontend-js

entities:
	sea-orm-cli generate entity -l -o packages/entities/src --with-serde both --enum-extra-derives "schemars::JsonSchema"

frontend_openapi:
	bun run --cwd packages/typescript-openapi openapi-generator-cli generate

build_frontend: build_analysis build_client_common build_crypto frontend_openapi
	bun run --cwd packages/frontend build

build_frontend_extra: 
	TS_RS_EXPORT_DIR=../client-js-extra-types cargo test -p palform-client-common --no-default-features --features frontend-js
