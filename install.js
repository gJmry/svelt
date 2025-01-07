const fs = require('fs');
const path = require('path');
const os = require('os');

let binPath;

if (os.platform() === 'win32') {
    binPath = path.resolve(__dirname, 'bin', 'svelt.exe');
} else if (os.platform() === 'linux') {
    binPath = path.resolve(__dirname, 'bin', 'svelt-linux');
} else if (os.platform() === 'darwin') {
    binPath = path.resolve(__dirname, 'bin', 'svelt-mac');
}

if (fs.existsSync(binPath)) {
    console.log('svelt has been installed successfully!');
} else {
    console.error('Error: svelt binary is missing.');
    process.exit(1);
}
