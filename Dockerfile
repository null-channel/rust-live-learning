FROM rust

COPY . .
ENV PORT=9000
ENV DATABASE_URL="sqlite:todos.db"
RUN cargo build --bin todo --release
CMD ["./target/release/todo"]

