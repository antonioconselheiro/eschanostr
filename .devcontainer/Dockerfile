FROM rust

WORKDIR /eschanostr

RUN cargo install cargo-edit

# teclado português com horário brasileiro
ENV LANG=C.UTF-8
ENV LANGUAGE=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV TZ=America/Sao_Paulo
ENV DEBUG_COLORS=true
ENV FORCE_COLOR=true

CMD [ "/bin/bash" ]
