const os = require('os');
const ref = require('ref');
const ffi = require('ffi');
const path = require('path');
const logger = require('debug');
const injector = require('node-dll-injector');
const downloader = require('./download');
//
const {CACHE_DIR, download} = downloader;
const BIN_DIR = path.join(CACHE_DIR, 'bin');
const ENTRY_FILE = path.join(BIN_DIR, 'scaffold_wizard.dll');
//
exports.inquire = questions => download().then(() => {
    downloader.downloadUrl; // eslint-disable-line no-unused-expressions
    injectZlib1();
    const log = logger('scaffold-wizard:inquire');
    log('BIN_DIR=', BIN_DIR);
    log('ENTRY_FILE=', ENTRY_FILE);
    const scaffoldWizard = ffi.Library(ENTRY_FILE, {
        inquire: ['string', ['string', 'string', 'string']]
    });
    return scaffoldWizard.inquire(reformQuestions(questions), BIN_DIR, ref.NULL_POINTER);
});
exports.inquireAsync = questions => download().then(() => new Promise((resolve, reject) => {
    downloader.downloadUrl; // eslint-disable-line no-unused-expressions
    injectZlib1();
    const log = logger('scaffold-wizard:inquire');
    log('BIN_DIR=', BIN_DIR);
    log('ENTRY_FILE=', ENTRY_FILE);
    const scaffoldWizard = ffi.Library(ENTRY_FILE, {
        inquireAsync: ['void', ['string', 'string', 'string', 'pointer']]
    });
    const nativeCallback = ffi.Callback('void', ['string', 'string'], finishedBuilder((err, answers) => {
        if (err) {
            reject(err);
        } else {
            resolve(answers);
        }
    }));
    scaffoldWizard.inquireAsync(reformQuestions(questions), BIN_DIR, ref.NULL_POINTER, nativeCallback);
}));
function reformQuestions(questions){
    let q = questions;
    if (typeof q === 'string') {
        q = JSON.parse(q);
    }
    q = {
        prompts: q
    };
    return JSON.stringify(q);
}
function injectZlib1(){
    const log = logger('scaffold-wizard:inject-dll');
    log('platform=', os.platform(), 'cpu-arch=', os.arch());
    if (os.platform() === 'win32' && os.arch() === 'x64') {
        const zlib1 = path.join(CACHE_DIR, 'bin/zlib1.dll');
        log('zlib1=', zlib1, 'process-id=', process.pid);
        const isNodeRunning = injector.isProcessRunningPID(process.pid);
        if (isNodeRunning) {
            const success = injector.injectPID(process.pid, zlib1);
            if (success !== 0) {
                throw new Error(`${zlib1} 注入失败：${success}, ${process.pid}`);
            }
        }
        log('注入', zlib1, '给进程', process.pid);
    } else {
        log('不需要注入 zlib1');
    }
}
function finishedBuilder(callback){
    let timerId;
    const holding = () => {
        timerId = setTimeout(holding, 1000 * 60 * 60 * 24);
    };
    holding();
    return (err, answers) => {
        clearTimeout(timerId);
        return callback(err, answers);
    };
}
