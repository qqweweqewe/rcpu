echo "Building project..."
cargo build --release | echo
if [ $? -ne 0 ]; then
  echo "Cargo error" >&2
  exit 1
fi

echo "Installing executable..."
sudo useradd -r -s /usr/sbin/nologin rcpu
sudo cp target/release/rcpu /usr/local/bin/
sudo cp rcpu.service /etc/systemd/system/
echo "\n"

echo "Integrating systemd service..."
sudo systemctl start rcpu

if [ $? -ne 0 ]; then
  echo "Service integration error" >&2
  exit 1
fi