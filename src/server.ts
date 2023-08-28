import express from 'express';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);

const __dirname = path.dirname(__filename);

const distFolder = path.join(__dirname, '../dist');

console.log(distFolder);

const app = express();
const port = process.env.PORT ?? 8080;

app.use('/', express.static(distFolder));

app.get('/*', (_request, response) => {
    response.sendFile(path.join(distFolder, "src/index.html"));
});

app.listen(port, () => {
    console.log(`Server is listening on port ${port}`);
})