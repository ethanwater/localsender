launch:
	@cargo run -- launch

upload:
	cargo run -- upload 192.168.1.172:8080 Makefile     

upload-force:
	cargo run -- upload 10.162.34.191:8080 Makefile force 

download:
	cargo run -- download  192.168.1.171:8080 Girl-in-Space.jpg 
