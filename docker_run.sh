#!/usr/bin/env bash

./docker_build.sh
docker run --rm -it --name crust_boot_you_container --user crust -w /home/crust crust_boot_you_app /bin/bash
