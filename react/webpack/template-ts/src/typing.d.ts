declare namespace NodeJS {
  interface Require {
    context(path: string, deep?: boolean, filter?: RegExp, mode?: 'sync' | 'lazy' | 'eager'): any;
  }
}