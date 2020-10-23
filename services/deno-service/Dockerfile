FROM hayd/alpine-deno:1.3.1

# The port that your application listens to.
EXPOSE 1993

WORKDIR /app

# Prefer not to run as root.
USER deno

# Cache the dependencies as a layer (the following two steps are re-run only when deps.ts is modified).
# Ideally cache deps.ts will download and compile _all_ external files used in main.ts.
COPY . .
RUN deno cache deps.ts

# Compile the main app so that it doesn't need to be compiled each startup/entry.
RUN deno cache server.tsx

CMD ["run", "--allow-net","--allow-read", "server.tsx"]