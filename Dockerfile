FROM rustlang/rust:nightly
 
RUN cargo install grcov
ENV CARGO_INCREMENTAL=0
ENV RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
#RUN cargo test --manifest-path /autograder/assignment/Cargo.toml
#RUN apt-get update && apt-get install -y zip
#RUN zip -0 /ccov.zip `find /autograder \( -name "submission*.gc*" \) -print`
COPY autograder /autograder
RUN cargo build --manifest-path /autograder/assignment/Cargo.toml
COPY run_autograder .
CMD ./run_autograder
