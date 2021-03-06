install:
	@bash ./scripts/preinstall.sh

	@echo -n "Compiling server (This can take some time...): "
	@cargo build --release -Z unstable-options --out-dir /usr/bin &> /dev/null
	@echo "Done."

	@echo -n "Building frontend: "
	@yarn --cwd client &> /dev/null
	@yarn --cwd client build &> /dev/null
	@echo "Done."

	@cp -r ./www /etc/ferrispanel/www

	@bash ./scripts/postinstall.sh

	@cp ./systemd/ferrispanel.service /lib/systemd/system


remove:
	@userdel ferrispanel

	@rm -rf /etc/ferrispanel
	@rm -rf /run/ferrispanel
	@rm /usr/bin/ferrispanel
	@rm /lib/systemd/system/ferrispanel.service

	@systemctl daemon-reload
	@systemctl reset-failed

	@echo -e "\033[0;32mSuccessfully uninstalled FerrisPanel\033[0m"

dev:
	yarn --cwd client build
	cargo run -- start
