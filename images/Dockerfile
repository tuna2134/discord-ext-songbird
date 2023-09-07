FROM ghcr.io/pyo3/maturin

WORKDIR /

RUN yum install -y opus-devel

WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]
