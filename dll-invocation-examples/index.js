const fs = require('fs');
const ref = require('ref');
const ffi = require('ffi');
const util = require('util');
const path = require('path');
const os = require('os');
//
const unlink = util.promisify(fs.unlink);
const symlink = util.promisify(fs.symlink);
const readFile = util.promisify(fs.readFile);
const lstat = util.promisify(fs.lstat);
//
const homeDir = path.resolve(__dirname, '../target/setup-lib');
const isAsync = ~process.argv.indexOf('--async-mode');
//
(async () => {
    const dllFile = path.join(homeDir, 'bin/scaffold_wizard.dll');
    const dllDir = path.dirname(dllFile);
    let zlib1Target;
    if (os.platform() === 'win32') {
        const zlib1Src = path.join(dllDir, 'zlib1.dll');
        zlib1Target = path.join(path.dirname(process.execPath), 'zlib1.dll');
        await lstat(zlib1Target).then(async stats => {
            if (stats.isSymbolicLink()) {
                await unlink(zlib1Target);
                await symlink(zlib1Src, zlib1Target, 'file');
            }
        }, () => symlink(zlib1Src, zlib1Target, 'file'));
        //
        process.env.PATH = `${dllDir}${path.delimiter}${process.env.PATH}`;
    }
    const questionsFile = path.join(homeDir, 'assets/prompt-manifest.json');
    const questions = await readFile(questionsFile, {encoding: 'utf8'});
    // 关键代码块
    const scaffoldWizard = ffi.Library(dllFile, {
        inquire: ['string', ['string', 'string', 'string']],
        inquireAsync: ['void', ['string', 'string', 'string', 'pointer']]
    });
    if (isAsync) {
        scaffoldWizard.inquireAsync(questions, dllDir, ref.NULL_POINTER, ffi.Callback('void', ['string', 'string'], finishedBuilder(finished)));
    } else {
        finished(null, scaffoldWizard.inquire(questions, dllDir, ref.NULL_POINTER));
    }
    async function finished(err, answers){
        if (err == null) {
            if (zlib1Target) {
                await unlink(zlib1Target);
            }
            console.info('被收集的答案包括', answers);
        } else {
            console.error('错误消息', err);
        }
    }
})();
function finishedBuilder(callback){
    let timerId;
    const holding = () => {
        timerId = setTimeout(holding, 1000 * 60 * 60);
    };
    holding();
    return (err, answers) => {
        clearTimeout(timerId);
        return callback(err, answers);
    };
}
