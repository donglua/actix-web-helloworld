FROM gitpod/workspace-postgres

USER gitpod

RUN bash -cl "rustup toolchain install nightly && rustup default nightly"
