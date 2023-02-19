"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const readline = __importStar(require("readline"));
const rl = readline.createInterface({
    input: process.stdin,
});
let t, n, m;
let t1 = '';
let t2 = '';
rl.on('line', (line) => {
    if (!t) {
        t = parseInt(line, 10);
    }
    else if (!n && !m) {
        [n, m] = line.split(' ').map(Number);
    }
    else if (!t1) {
        t1 = line;
    }
    else if (!t2) {
        t2 = line;
    }
    if (t1 && t2) {
        t--;
        isBeautiful(t1, t2);
        (t1 = ''), (t2 = ''), (n = m = 0);
        if (!t)
            rl.close();
    }
});
function isBeautiful(t1, t2) {
    // const str = t1 + t2.split('').map((_, i) => t2.at(~i)).join('');
    const str = t1 + t2.split('').reverse().join('');
    let n = 0;
    for (let i = 0; i < str.length; i++) {
        if (str[i] == str[i + 1])
            n++;
        if (n >= 2) {
            return console.log('NO');
        }
    }
    return console.log('YES');
}
