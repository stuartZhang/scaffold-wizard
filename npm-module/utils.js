const libPrefix = process.platform === 'darwin' ? 'lib' : '';
const libExtName = process.platform === 'darwin' ? '.dylib' : '.dll';
exports.dyLibFullName = `${libPrefix}scaffold_wizard${libExtName}`;
const binPrefix = '';
const binExtName = process.platform === 'win32' ? '.exe' : '';
exports.binFullName = `${binPrefix}scaffold-wizard${binExtName}`;
exports.defaultTag = process.platform === 'win32' ? 'basic3' : 'basic4';
exports.targetTriple = `${process.platform}-${process.arch}`;
