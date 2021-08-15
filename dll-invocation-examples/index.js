const fs = require('fs');
const ref = require('ref');
const ffi = require('ffi');
const util = require('util');
const path = require('path');
const os = require('os');
const safeRequire = require('safe-require');
//
const readFile = util.promisify(fs.readFile);
//
const homeDir = path.resolve(__dirname, '../target/setup-lib');
const isAsync = ~process.argv.indexOf('--async-mode');
//
(async () => {
    const dllFile = path.join(homeDir, 'bin/scaffold_wizard.dll');
    const dllDir = path.dirname(dllFile);
    if (os.platform() === 'win32' && os.arch() === 'x64') {
        const injector = safeRequire('node-dll-injector');
        if (injector) {
            const zlib1Src = path.join(dllDir, 'zlib1.dll');
            const isNodeRunning = injector.isProcessRunningPID(process.pid);
            if (isNodeRunning) {
                const success = injector.injectPID(process.pid, zlib1Src);
                if (success !== 0) {
                    throw new Error(`${zlib1Src} 注入失败：${success}, ${process.pid}`);
                }
            }
        }
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
    function finished(err, answers){
        if (err == null) {
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
