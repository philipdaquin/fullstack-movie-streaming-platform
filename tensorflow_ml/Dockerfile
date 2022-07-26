FROM python:3.7


    # apt-get install -y python-pip python-dev
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang  \
    llvm-dev libclang-dev openssl cmake 
COPY . ./tensorflow_ml

COPY /tensorflow_ml/requirements.txt ./tensorflow_ml

WORKDIR /tensorflow_ml
# We copy just the requirements.txt first to leverage Docker cache
RUN pip install -r requirements.txt

ENTRYPOINT [ "python" ]
