install:
	bash ./scripts/preinstall.sh

	cargo build --release -Z unstable-options --out-dir /usr/bin
	yarn --cwd client build

	cp -r ./www /etc/ferrispanel/www

	bash ./scripts/postinstall.sh


remove:
	userdel ferrispanel

	rm -rf /etc/ferrispanel
	rm -rf /run/ferrispanel
	rm /usr/bin/ferrispanel

	echo "Successfully uninstalled FerrisPanel"