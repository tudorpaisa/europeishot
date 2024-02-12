FROM rust:1.67

COPY ./ ./

# RUN apt-get update && apt-get install -y cron curl
#
# COPY update-temps-crontab /etc/cron.d/update-temps-crontab
#
# RUN chmod 0644 /etc/cron.d/update-temps-crontab
#
# RUN crontab /etc/cron.d/update-temps-crontab
#
# RUN touch /var/log/cron.log
#
# CMD cron && tail -f /var/log/cron.log

RUN cargo build --release

CMD ["./target/release/backend"]
