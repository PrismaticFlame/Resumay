import { readFileSync } from 'node:fs';

const v = 'v' + JSON.parse(readFileSync('package.json', 'utf8')).version;
if (v !== process.env.GITHUB_REF_NAME) {
    console.error(`Tag ${process.env.GITHUB_REF_NAME} does not match package.json (${v})`);
    process.exit(1);
}