FROM rust:1.71.0 as builder

WORKDIR /usr/src/myapp
COPY . .

RUN curl -fsSL https://deno.land/x/install/install.sh | sh
ENV PATH="/root/.deno/bin:${PATH}"

RUN apt-get update && apt-get install -y libclang-dev llvm-dev

RUN ./build.ts release use_v8

CMD ["./target/release/elsa", "uring.ts"]
