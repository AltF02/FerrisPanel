if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

echo -n "Creating group: "
groupadd -r ferrispanel
echo "Done."

echo -n "Creating user: "
useradd -r -g ferrispanel -m -d /etc/ferrispanel ferrispanel
echo "Done."

echo -n "Installing sqlx cli: "
cargo install sqlx-cli --no-default-features --features postgres &> /dev/null
echo "Done."
