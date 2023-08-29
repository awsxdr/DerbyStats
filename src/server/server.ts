import express from 'express';
import path from 'path';
import { fileURLToPath } from 'url';
import { ScoreboardConnector } from './scoreboard-connector.js';
import SocketServer from './api/socket-server.js';
import { CumulativeScoreController } from './api/cumulative-score.js';
import expressWs from 'express-ws';
import { parseArgs } from 'node:util';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const distFolder = path.join(__dirname, '../ui/dist');
console.log(distFolder);

const app = expressWs(express()).app;

const { values: { scoreboardAddress, portString }, positionals: {} } = parseArgs({
    options: {
        scoreboardAddress: {
            type: "string",
            short: "s"
        },
        portString: {
            type: "string",
            short: "p"
        }
    }
});

const port = parseInt(portString || "8080");

app.use('/', express.static(distFolder));

const connector = new ScoreboardConnector(scoreboardAddress || "localhost:8000");

const connectPromise = new Promise((resolve) => {
    connector.on("Connect", () => {
        const socketServer = new SocketServer(app);
        new CumulativeScoreController(app, connector, socketServer);

        resolve(socketServer);
    });
});

await connectPromise;

app.get('/*', (_request, response) => {
    console.log("1");
    response.sendFile(path.join(distFolder, "index.html"));
});

app.listen(port, () => {
    console.log(`Server is listening on port ${port}`);
});