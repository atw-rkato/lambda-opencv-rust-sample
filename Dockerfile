ARG ACCOUNT_ID
FROM ${ACCOUNT_ID}.dkr.ecr.ap-northeast-1.amazonaws.com/lambda-opencv-rust:4.5.5-1.61

WORKDIR /code
COPY src /code/src
COPY Cargo.toml Cargo.lock rust-toolchain /code/
RUN cargo test  --release -j 12 --no-fail-fast -- --nocapture
RUN cargo build --release -j 12
RUN cp target/release/bootstrap ${LAMBDA_RUNTIME_DIR}
RUN rustup self uninstall -y

WORKDIR ${LAMBDA_TASK_ROOT}
RUN rm -rf /code

# ハンドラ名は使われないが、ないとエラーになるので適当な文字列を入れておく
CMD ["bootstrap.is.real.handler"]
