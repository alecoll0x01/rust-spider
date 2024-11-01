#!/bin/bash

sudo mkdir -p /opt/web-crawler
sudo mkdir -p /opt/web-crawler/bin
sudo cp target/release/web-crawler /opt/web-crawler/bin/
sudo ln -sf /opt/web-crawler/bin/web-crawler /usr/local/bin/web-crawler
sudo chmod +x /opt/web-crawler/bin/web-crawler
sudo chmod +x /usr/local/bin/web-crawler

echo "Rust-Spider Web Crawler instalado com sucesso!"
echo "Use 'web-crawler --help' para ver as opções disponíveis"