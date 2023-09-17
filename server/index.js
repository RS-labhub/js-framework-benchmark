import { buildServer } from "./app.js";

const PORT = 8080;

const server = buildServer();

try {
  await server.listen({ port: PORT });
  console.log(`Server running on port ${PORT}`);
} catch (error) {
  server.log.error(error);
  process.exit(1);
}
