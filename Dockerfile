FROM debian:stable-slim

ARG env=debug

# Install deps, add user and cleanup
RUN apt-get update \
  && apt-get install -y wget gnupg2 \
  && wget -q https://www.postgresql.org/media/keys/ACCC4CF8.asc -O - | apt-key add - \
  && sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ stretch-pgdg main" >> /etc/apt/sources.list.d/pgdg.list' \
  && apt-get update \
  && apt-get update && apt-get install -y libpq5 libmariadbclient18 \
  && apt-get purge -y wget \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/ \
  && mkdir -p /app/config \
  && adduser --disabled-password --gecos "" --home /app --no-create-home -u 5000 app

WORKDIR /app

COPY target/$env/gateway_runner /app
COPY config /app/config
RUN chown -R app: /app

EXPOSE 8000
USER app

ENTRYPOINT ["/app/gateway_runner"]
