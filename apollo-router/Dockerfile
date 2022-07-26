FROM rust:1.60

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y npm curl
RUN curl -sSL https://rover.apollo.dev/nix/latest | sh

WORKDIR /apollo-router/
# Update the supergraph.graphql
RUN rover supergraph compose --config supergraph.yaml > supergraph.graphql
# Run Apollo ROuter
RUN ./router -c router.yaml --log info  --supergraph supergraph.graphql   

CMD ["apollo-router"]
EXPOSE 4000