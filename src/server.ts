import express from 'express';

const app = express();
const port = process.env.PORT ?? 8080;

app.get('/', (_request, response) => {
    response.status(200).send("");
});

app.listen(port, () => {
    console.log(`Server is listening on port ${port}`);
})