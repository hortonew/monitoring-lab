FROM golang:latest as builder
WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download
COPY main.go .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o vault-load-test .

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /app/vault-load-test .
RUN chmod +x ./vault-load-test
CMD ["./vault-load-test"]
