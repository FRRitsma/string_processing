#Testing the installation of the python interface of the rust function
FROM python:3.11-alpine
LABEL authors="frrit"

# Install git:
RUN apk add git

ENTRYPOINT ["top", "-b"]


#pip install git+https://github.com/FRRitsma/string_processing/releases/download/v0/string_processing-0.1.0-cp311-cp311-win_amd64.whl