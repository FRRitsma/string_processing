#Testing the installation of the python interface of the rust function
FROM python:3.10-slim
LABEL authors="frrit"

# Install string_processing
RUN apt-get update && apt-get install -y --no-install-recommends git
RUN pip install --upgrade pip
#RUN pip install https://github.com/FRRitsma/string_processing/releases/download/v1/string_processing-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl
RUN pip install --extra-index-url https://frritsma.github.io/string_processing/simple/string_processing/ string_processing

RUN pip install pytest
COPY python_testing python_testing
RUN cd python_testing; python -m pytest
ENTRYPOINT ["tail", "-f", "/dev/null"]
