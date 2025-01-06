const fs = require('fs');
const path = require('path');

const binPath = path.resolve(__dirname, 'bin', 'svelt.exe');

if (fs.existsSync(binPath)) {
    console.log('svelt.exe has been installed successfully!');
} else {
    console.error('Error: svelt.exe is missing.');
    process.exit(1);
}
