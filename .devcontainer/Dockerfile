FROM ubuntu

RUN apt-get update
run apt install -y build-essential
RUN apt install -y curl
RUN apt install -y git
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


