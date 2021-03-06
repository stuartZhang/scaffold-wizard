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
let nativeCallback;
//
exports.inquire = questions => download().then(injectZlib1).then(() => {
    downloader.downloadUrl; // eslint-disable-line no-unused-expressions
    const log = logger('scaffold-wizard:inquire');
    log('BIN_DIR=', BIN_DIR);
    log('ENTRY_FILE=', ENTRY_FILE);
    const scaffoldWizard = ffi.Library(ENTRY_FILE, {
        inquire: ['string', ['string', 'string', 'string']]
    });
    return JSON.parse(scaffoldWizard.inquire(reformQuestions(questions), BIN_DIR, ref.NULL_POINTER));
});
exports.inquireAsync = questions => download().then(injectZlib1).then(() => new Promise((resolve, reject) => {
    downloader.downloadUrl; // eslint-disable-line no-unused-expressions
    const log = logger('scaffold-wizard:inquire');
    log('BIN_DIR=', BIN_DIR);
    log('ENTRY_FILE=', ENTRY_FILE);
    const scaffoldWizard = ffi.Library(ENTRY_FILE, {
        inquireAsync: ['void', ['string', 'string', 'string', 'pointer']]
    });
    nativeCallback = ffi.Callback('void', ['string', 'string'], finishedBuilder((err, answers) => {
        if (err) {
            reject(new Error(err));
        } else {
            resolve(JSON.parse(answers));
        }
        nativeCallback = null;
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
    return new Promise((resolve, reject) => {
        const log = logger('scaffold-wizard:inject-dll');
        log('platform=', os.platform(), 'cpu-arch=', os.arch());
        if (os.platform() === 'win32' && os.arch() === 'x64') {
            const zlib1 = path.join(CACHE_DIR, 'bin/zlib1.dll');
            log('zlib1=', zlib1, 'process-id=', process.pid);
            const isNodeRunning = injector.isProcessRunningPID(process.pid);
            if (isNodeRunning) {
                const success = injector.injectPID(process.pid, zlib1);
                if (success !== 0) {
                    reject(new Error(`${zlib1} ???????????????${success}, ${process.pid}`));
                    return;
                }
            }
            log('??????', zlib1, '?????????', process.pid);
            setTimeout(resolve, 0);
        } else {
            log('??????????????? zlib1');
            resolve();
        }
    });
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
