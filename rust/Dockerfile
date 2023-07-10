# Use the main rust Docker image
FROM rust:1.67.0-slim-bullseye

# copy app into docker image
COPY . /app

# Set the workdirectory
WORKDIR /app

# build the app
RUN cargo build --release

# start the application
CMD ["./target/release/cloud-native-service-template"]

