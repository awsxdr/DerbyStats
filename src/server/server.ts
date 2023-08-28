import express from 'express';
import path from 'path';
import { fileURLToPath } from 'url';
import { ScoreboardConnector } from './scoreboard-connector.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const distFolder = path.join(__dirname, '../ui/dist');
console.log(distFolder);

const app = express();
const port = process.env.PORT ?? 8080;

const topics = [
    "ScoreBoard.Version(release)",
    "ScoreBoard.CurrentGame.Clock(Period)",
    "ScoreBoard.CurrentGame.CurrentPeriodNumber",
    "ScoreBoard.CurrentGame.Period(*).Jam(*).TeamJam(*).TotalScore",
    "ScoreBoard.CurrentGame.Team(*).Skater(*).Penalty(*)",
];

app.use('/', express.static(distFolder));

app.get('/*', (_request, response) => {
    response.sendFile(path.join(distFolder, "index.html"));
});

const connector = new ScoreboardConnector('192.168.86.33:8000');

connector.on("Connect", () => {
    topics.forEach(s => {
        connector.listenForTopic(s); 
    });
});

connector.on("Update", (_state) => {
});

app.listen(port, () => {
    console.log(`Server is listening on port ${port}`);
})