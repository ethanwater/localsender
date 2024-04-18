launch:
	clear
	cargo run -- launch

upload:
	clear
	cargo run -- upload 192.168.1.171:8080 Makefile     

upload-force:
	clear
	cargo run -- upload 192.168.1.171:8080 Makefile force 

download:
	clear
	cargo run -- download 192.168.1.171:8080 Girl-in-Space.jpg
