FROM ubuntu:latest

RUN apt-get update && apt-get -y install -qq --force-yes cron && apt-get -y install curl

# Copy curl-cron file to the cron.d directory
COPY curl-cron /etc/cron.d/curl-cron
 
# Give execution rights on the cron job
RUN chmod 0644 /etc/cron.d/curl-cron

# Apply cron job
RUN crontab /etc/cron.d/curl-cron
 
# Create the log file to be able to run tail
RUN touch /var/log/cron.log
 
# Run the command on container startup
CMD cron && tail -f /var/log/cron.log