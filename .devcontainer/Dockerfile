FROM minizinc/minizinc:2.7.6
RUN apt update && \
    apt install -y make sudo
ENV SHELL bash
ARG USERNAME=node
RUN useradd -m $USERNAME && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

RUN echo "alias m='minizinc --solver gecode --all-solutions'" >> /home/node/.bashrc

USER $USERNAME
WORKDIR /home/node/works
COPY . /home/node/works
