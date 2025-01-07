const fs = require('fs');
const path = require('path');
const os = require('os');

let binPath;

if (os.platform() === 'win32') {
    if (os.arch() === 'x64') {
        binPath = path.resolve(__dirname, 'bin', 'svelt-win64.exe');
    } else if (os.arch() === 'ia32') {
        binPath = path.resolve(__dirname, 'bin', 'svelt-win32.exe');
    }
} else if (os.platform() === 'linux') {
    binPath = path.resolve(__dirname, 'bin', 'svelt-linux-x86_64');
} else if (os.platform() === 'darwin') {
    binPath = path.resolve(__dirname, 'bin', 'svelt-mac');
}

if (fs.existsSync(binPath)) {
    console.log('svelt has been installed successfully!');
} else {
    console.error('Error: svelt binary is missing.');
    process.exit(1);
}
