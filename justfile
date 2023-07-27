app_name := "crust_boot_you" 
linux_gnu_target := "x86_64-unknown-linux-gnu"
relase_folder := "release"
tar_file_name := relase_folder / app_name + "_" + linux_gnu_target + ".gz.tar"
checksum := relase_folder / "checksum_sha256.txt"

default: docker-try

# Builds image for playground of appliaction
docker-build: 
  docker build -t crust_boot_you_app .

# Runs application in a already built playground container 
docker-run:
  docker run --rm -it --name crust_boot_you_container --user crust -w /home/crust crust_boot_you_app /bin/bash

docker-try: docker-build docker-run

local-install:
  cargo install --path . --force

local-release: 
  cargo build --target {{ linux_gnu_target }} --release
  rm -fr {{ relase_folder }}
  mkdir {{ relase_folder }}
  tar --create --gzip --file {{ tar_file_name }} README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE -C ./target/release/ {{ app_name }}
  sha256sum {{ tar_file_name }} > {{ checksum }}
