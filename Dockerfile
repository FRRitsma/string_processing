FROM ubuntu:latest
LABEL authors="frrit"

ENTRYPOINT ["top", "-b"]