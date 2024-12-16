import { get, post, del, put } from './request';

const controllers = new Map<string, AbortController>();

/**
 * 通用取消请求方法
 * @param {Function} requestMethod 请求方法（如 get、post 等）
 * @param {string} url 请求路径
 * @param {object} payload 请求参数（GET, DELETE 使用 params，其他方法使用 data）
 * @param {object} options 其他配置项
 * @param {string} key 请求标识符（可选）
 * @returns {Promise} 请求 Promise
 */
const withCancel = async <T>(
  requestMethod: (
    url: string,
    payload: object,
    config: RequestInit
  ) => Promise<T>,
  url: string,
  payload: object = {},
  options: object = {},
  key?: string
): Promise<T> => {
  const uniqueKey = key || url;

  // 取消之前的请求
  if (controllers.has(uniqueKey)) {
    controllers.get(uniqueKey)?.abort();
  }

  // 创建新的控制器
  const controller = new AbortController();
  controllers.set(uniqueKey, controller);

  // 发起请求
  const config = {
    ...options,
    signal: controller.signal,
  };

  return requestMethod(url, payload, config).finally(() => {
    controllers.delete(uniqueKey);
  });
};

export const getWithCancel = (
  url: string,
  params?: object,
  options?: RequestInit,
  key?: string
) => withCancel(get, url, params, options, key);

export const postWithCancel = (
  url: string,
  data?: object,
  options?: RequestInit,
  key?: string
) => withCancel(post, url, data, options, key);

export const delWithCancel = (
  url: string,
  params?: object,
  options?: RequestInit,
  key?: string
) => withCancel(del, url, params, options, key);

export const putWithCancel = (
  url: string,
  data?: object,
  options?: RequestInit,
  key?: string
) => withCancel(put, url, data, options, key);

export const cancelRequest = (key: string) => {
  if (controllers.has(key)) {
    controllers.get(key)?.abort();
    controllers.delete(key);
  }
};
