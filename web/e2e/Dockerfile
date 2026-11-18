FROM mcr.microsoft.com/playwright:v1.65.1-noble

# unzip is needed for deno install
RUN apt-get update \
    && apt-get install -y --no-install-recommends unzip

RUN curl -fsSL https://deno.land/install.sh | sh

ENV PLAYWRIGHT_BROWSERS_PATH="/ms-playwright"
ENV DENO_INSTALL="/root/.deno"
ENV PATH="$DENO_INSTALL/bin:$PATH"

WORKDIR /app

COPY . .

RUN deno install

CMD ["deno", "run", "-A", "npm:@playwright/test", "test", "--reporter", "html"]
