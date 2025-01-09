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

declare module 'virtual-routes' {
  export const getRoutes: () => {
    routes: Record<
      string,
      {
        id: string;
        parentId?: string;
        path: string;
        isLayout?: boolean;
        [key: string]: any;
      }
    >;
    routeComponents: Record<string, React.ComponentType<any>>;
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
