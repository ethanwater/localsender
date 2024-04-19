launch:
	@cargo run -- launch

upload:
	cargo run -- upload 10.162.34.191:8080 Makefile     

upload-force:
	cargo run -- upload 10.162.34.191:8080 Makefile force 

download:
	cargo run -- download 192.168.200.48:8080 Makefile 
