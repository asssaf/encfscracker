docker build -t asssaf/encfscrack-builder -f docker/Dockerfile .

docker run --rm -it \
	-v "${PWD}:/work" \
        --workdir /work \
	asssaf/encfscrack-builder \
	cargo build
