ARG APP_NAME=kafka_db

# Build stage
FROM golang:1.21 as build
ARG APP_NAME
ENV APP_NAME=$APP_NAME
WORKDIR /app
COPY . .
RUN go mod download
RUN go build -o /app/kafka_db ./cmd

# Production stage
FROM alpine:latest as production
ARG APP_NAME
ENV APP_NAME=$APP_NAME
WORKDIR /root/
COPY --from=build /app/kafka_db ./
CMD ["./kafka_db"]
