FROM fedora
RUN dnf install gtk4-devel -y
COPY generator.sh /.
RUN chmod +x generator.sh
ENTRYPOINT ["/bin/bash","/generator.sh"]