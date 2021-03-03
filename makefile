install:
	@bash ./scripts/preinstall.sh

	@cargo build --release -Z unstable-options --out-dir /usr/bin
	@yarn --cwd client
	@yarn --cwd client build

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

	@echo -e "\033[0;32mSuccessfully uninstalled FerrisPanel\033[0m"