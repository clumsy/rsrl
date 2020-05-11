FROM gitpod/workspace-full

RUN sudo apt-get update \
 && sudo apt-get install -y \
    gfortran \
    openblas \
 && sudo rm -rf /var/lib/apt/lists/*
