# Mosquitto broker image for the Thermagate dev container.
# The config is baked in rather than bind-mounted: a single-file bind mount
# silently becomes an empty directory if the stack is ever brought up before
# the file exists, which then crash-loops the broker. See ../CONTRIBUTING.md.
FROM eclipse-mosquitto:2
COPY mosquitto.conf /mosquitto/config/mosquitto.conf
