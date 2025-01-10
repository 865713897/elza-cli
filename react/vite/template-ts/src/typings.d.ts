declare module '*.svg';
declare module '*.png';
declare module '*.css';

declare module 'virtual:routes' {
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
