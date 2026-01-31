FROM denoland/deno:2.6.7

ENV PATH="/root/.deno/bin:${PATH}"

RUN deno x -y playwright install --with-deps

WORKDIR /app

COPY . .

RUN deno install

CMD ["deno", "run", "-A", "npm:@playwright/test", "test", "--reporter", "html"]
