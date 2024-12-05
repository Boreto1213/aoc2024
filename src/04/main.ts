import * as fs from 'fs';

function readFileSync(filePath: string): string {
  return fs.readFileSync(filePath, 'utf-8');
}

// Usage
const data = readFileSync('./data.txt');
console.log(data);

