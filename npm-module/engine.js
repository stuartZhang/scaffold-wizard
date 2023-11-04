const path = require('path');
const ffi = require('ffi-napi');
const logger = require('debug');
const downloader = require('./download');
const {dyLibFullName} = require('./utils');
//
const {CACHE_DIR} = downloader;
const BIN_DIR = path.join(CACHE_DIR, 'bin');
const ENTRY_FILE = path.join(BIN_DIR, dyLibFullName);
let nativeCallback;
exports.inquire = questions => {
    const log = logger('scaffold-wizard:inquire');
    log('BIN_DIR=', BIN_DIR);
    log('ENTRY_FILE=', ENTRY_FILE);
    const scaffoldWizard = ffi.Library(ENTRY_FILE, {
        inquire: ['string', ['string', 'string', 'string']]
    });
    return JSON.parse(scaffoldWizard.inquire(reformQuestions(questions), BIN_DIR, ffi.types.NULL_POINTER));
};
exports.inquireAsync = questions => new Promise((resolve, reject) => {
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
    scaffoldWizard.inquireAsync(reformQuestions(questions), BIN_DIR, ffi.types.NULL_POINTER, nativeCallback);
});
if (typeof process.send === 'function') {
    process.on('message', request => {
        try {
            if (request.action === 'questions') {
                process.send({
                    type: 'success',
                    data: exports.inquire(request.data)
                });
            } else {
                process.send({
                    type: 'failure',
                    data: new Error(`未被注册的 action=${request.action}`)
                });
            }
        } catch (err) {
            process.send({
                type: 'failure',
                data: `${err.message}\n${err.stack}`
            });
        }
    });
}
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
