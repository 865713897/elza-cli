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
  constructor(text) {
    this.text = text;
  }

  red() {
    this.text = colors.red + this.text + colors.reset;
    return this.text;
  }

  green() {
    this.text = colors.green + this.text + colors.reset;
    return this.text;
  }

  yellow() {
    this.text = colors.yellow + this.text + colors.reset;
    return this.text;
  }

  blue() {
    this.text = colors.blue + this.text + colors.reset;
    return this.text;
  }

  magenta() {
    this.text = colors.magenta + this.text + colors.reset;
    return this.text;
  }

  cyan() {
    this.text = colors.cyan + this.text + colors.reset;
    return this.text;
  }

  color256(num) {
    this.text = `\x1b[38;5;${num}m${this.text}\x1b[0m`;
    return this.text;
  }

  bold() {
    this.text = `${this.text}\x1b[1m`;
    return this.text;
  }
}

function style(text) {
  return new Style(text);
}

const logger = {};

logger.info = (msg) => {
  console.log(`${style('info').cyan()}  - ${msg}`);
};

logger.info_version = (msg) => {
  console.log(`${style('info').cyan()}  - ${style(msg).color256(14).bold()}`);
};

logger.event = (msg) => {
  console.log(`${style('event').color256(177)} - ${msg}`);
};

logger.ready = (msg) => {
  console.log(`${style('ready').green()} - ${msg}`);
};

logger.error = (msg) => {
  console.log(`${style('error').red()} - ${msg}`);
};

module.exports = logger;
