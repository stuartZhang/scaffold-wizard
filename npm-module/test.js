const path = require('path');
const fs = require('fs-extra');
const {inquire, inquireAsync} = require('./index');
//
const isAsync = ~process.argv.indexOf('--async-mode');
//
(async () => {
    const questionsFile = path.join(__dirname, './prompt-manifest.json');
    const questions = await fs.readFile(questionsFile, {encoding: 'utf8'});
    (isAsync ? inquireAsync : inquire)(JSON.parse(questions)).then(answer => {
        console.log('成功：', answer);
    }, err => {
        console.error('失败：', err);
    });
})();
