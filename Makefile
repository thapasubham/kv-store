SERVER_BIN := target/release/db_server.exe
SHELL_DIR  := rust-db-shell
SHELL_BIN  := $(SHELL_DIR)/bin/shell.exe
ADDR       := 127.0.0.1:5000

build: build-server build-shell

build-server:
	cargo build --release

build-shell:
	cd $(SHELL_DIR) && go build -o bin/shell.exe ./cmd

run-server:
	cargo run --release

run-shell:
	cd $(SHELL_DIR) && go run ./cmd

ifeq ($(OS),Windows_NT)
    FIX_SERVER_BIN = $(subst /,\,$(SERVER_BIN))
    FIX_SHELL_BIN  = $(subst /,\,$(SHELL_BIN))

    DEV_START_SERVER = $(FIX_SERVER_BIN)
    DEV_START_SHELL  = cmd.exe /C start "Go Shell" cmd /k "$(FIX_SHELL_BIN)"
    DEV_WAIT         = powershell -NoProfile -Command "Start-Sleep -Seconds 1"
else
    DEV_START_SERVER = ./$(SERVER_BIN)
    DEV_START_SHELL  = ./$(SHELL_BIN) &
    DEV_WAIT         = sleep 1
endif

dev: build
		$(DEV_START_SHELL)
		$(DEV_START_SERVER)
clean:
	cargo clean
