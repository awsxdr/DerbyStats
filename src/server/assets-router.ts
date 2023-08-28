import express from 'express';

export const router = express.Router();

const imageRegex = /\/.+\.(svg|png|jpe?g)$/;

router.get(imageRegex, (request, response) => {
    const filePath = request.path;
    response.redirect(303, `/src${filePath}`);
});
