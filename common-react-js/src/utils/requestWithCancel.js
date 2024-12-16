import { get, post, del, put } from './request';

const controllers = new Map();

/**
 * 通用取消请求方法
 * @param {Function} requestMethod 请求方法（如 get、post 等）
 * @param {string} url 请求路径
 * @param {object} payload 请求参数（GET, DELETE 使用 params，其他方法使用 data）
 * @param {object} options 其他配置项
 * @param {string} key 请求标识符（可选）
 * @returns {Promise} 请求 Promise
 */
const withCancel = async (
  requestMethod,
  url,
  payload = {},
  options = {},
  key
) => {
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
    signal: controller.signal,
    ...options,
  };

  return requestMethod(url, payload, config).finally(() => {
    controllers.delete(uniqueKey);
  });
};

// 导出带取消功能的请求方法
export const getWithCancel = (url, params, options, key) =>
  withCancel(get, url, params, options, key);

export const postWithCancel = (url, data, options, key) =>
  withCancel(post, url, data, options, key);

export const delWithCancel = (url, data, options, key) =>
  withCancel(del, url, data, options, key);

export const putWithCancel = (url, data, options, key) =>
  withCancel(put, url, data, options, key);

export const cancelRequest = (key) => {
  if (controllers.has(key)) {
    controllers.get(key)?.abort();
    controllers.delete(key);
  }
};
