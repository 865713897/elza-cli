const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  reset: '\x1b[0m',
};

class Style {
  constructor(text = '') {
    this.original = text;
    this.text = text;
  }

  apply(color) {
    this.text = color + this.text + colors.reset;
    return this;
  }

  red() {
    return this.apply(colors.red);
  }

  green() {
    return this.apply(colors.green);
  }

  yellow() {
    return this.apply(colors.yellow);
  }

  blue() {
    return this.apply(colors.blue);
  }

  magenta() {
    return this.apply(colors.magenta);
  }

  cyan() {
    return this.apply(colors.cyan);
  }

  color256(n) {
    this.text = `\x1b[38;5;${n}m` + this.text + colors.reset;
    return this;
  }

  bold() {
    this.text = '\x1b[1m' + this.text + colors.reset;
    return this;
  }

  toString() {
    return this.text;
  }
}

function style(text) {
  return new Style(text);
}

const logger = {
  info: (msg) => console.log(`${style('info').cyan()}  - ${msg}`),
  event: (msg) => console.log(`${style('event').color256(177)} - ${msg}`),
  error: (msg) => console.log(`${style('error').red()} - ${msg}`),
  ready: (msg) => console.log(`${style('ready').green()} - ${msg}`),
  warn: (msg) => console.log(`${style('warn').yellow()}  - ${msg}`),
};

module.exports = { logger, style };
