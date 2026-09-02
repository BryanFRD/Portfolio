import {
  AngularNodeAppEngine,
  createNodeRequestHandler,
  isMainModule,
  writeResponseToNodeResponse,
} from '@angular/ssr/node';
import express from 'express';
import { join } from 'node:path';

const browserDistFolder = join(import.meta.dirname, '../browser');
const apiUrl = process.env['API_URL'] ?? 'http://localhost:8080';

const app = express();
app.disable('x-powered-by');
const angularApp = new AngularNodeAppEngine();

app.use('/api', express.json(), async (req, res) => {
  try {
    const headers: Record<string, string> = { 'content-type': 'application/json' };
    if (req.headers.cookie) {
      headers['cookie'] = req.headers.cookie;
    }
    const csrfToken = req.headers['x-csrf-token'];
    if (typeof csrfToken === 'string') {
      headers['x-csrf-token'] = csrfToken;
    }
    const response = await fetch(`${apiUrl}/api${req.url}`, {
      method: req.method,
      headers,
      body: req.method === 'GET' || req.method === 'HEAD' ? undefined : JSON.stringify(req.body),
    });
    res.status(response.status);
    const contentType = response.headers.get('content-type');
    if (contentType) {
      res.setHeader('content-type', contentType);
    }
    const setCookie = response.headers.getSetCookie();
    if (setCookie.length > 0) {
      res.setHeader('set-cookie', setCookie);
    }
    res.send(Buffer.from(await response.arrayBuffer()));
  } catch {
    res.status(502).json({ error: 'API unreachable' });
  }
});

/**
 * Serve static files from /browser
 */
app.use(
  express.static(browserDistFolder, {
    maxAge: '1y',
    index: false,
    redirect: false,
  }),
);

/**
 * Handle all other requests by rendering the Angular application.
 */
app.use((req, res, next) => {
  angularApp
    .handle(req)
    .then((response) =>
      response ? writeResponseToNodeResponse(response, res) : next(),
    )
    .catch(next);
});

/**
 * Start the server if this module is the main entry point, or it is ran via PM2.
 * The server listens on the port defined by the `PORT` environment variable, or defaults to 4000.
 */
if (isMainModule(import.meta.url) || process.env['pm_id']) {
  const port = process.env['PORT'] || 4000;
  app.listen(port, (error) => {
    if (error) {
      throw error;
    }

    console.log(`Node Express server listening on http://localhost:${port}`);
  });
}

/**
 * Request handler used by the Angular CLI (for dev-server and during build) or Firebase Cloud Functions.
 */
export const reqHandler = createNodeRequestHandler(app);
