echo -n "Updating directories: "
chown ferrispanel:ferrispanel -R /etc/ferrispanel

mkdir /run/ferrispanel
chown ferrispanel:ferrispanel -R /run/ferrispanel
echo "Done."