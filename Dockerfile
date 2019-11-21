FROM rust

#WORKDIR /usr/src/ProjectOmegaDwarf
#COPY ./target/debug/ProjectOmegaDwarf /usr/src/ProjectOmegaDwarf/
#CMD  ["ProjectOmegaDwarf"]

WORKDIR /usr/src/ProjectOmegaDwarf
COPY . .

RUN cargo build

CMD cd target/debug && ./ProjectOmegaDwarf