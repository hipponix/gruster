add-dependencies:
	cargo add dotenv reqwest tokio serde \
		--features reqwest/json,tokio/full

build:
	cargo build --color always --verbose
