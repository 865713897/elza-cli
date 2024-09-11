declare namespace NodeJS {
  interface Require {
    context(
      path: string,
      deep?: boolean,
      filter?: RegExp,
      mode?: 'sync' | 'lazy' | 'eager'
    ): any;
  }
}

declare interface NodeModule {
  hot: {
    accept(path?: string, fn: () => void, callback?: () => void): void;
  };
}

declare module '*.scss' {
  const content: { [key: string]: string };
  export = content;
}

declare module '*.less' {
  const content: { [key: string]: string };
  export = content;
}
