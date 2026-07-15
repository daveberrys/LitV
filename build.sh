cargo build --target x86_64-unknown-linux-gnu --release
rm -rf /home/$USER/.local/bin/litv
mv target/x86_64-unknown-linux-gnu/release/litv /home/$USER/.local/bin/litv
chmod +x /home/$USER/.local/bin/litv
litv