import axios from 'axios';

axios.defaults.baseURL = 'http://localhost:3000';
axios.defaults.timeout = 5000;
axios.defaults.headers.common['Authorization'] = 'Bearer token';

// 请求拦截器
axios.interceptors.request.use(
  (config) => {
    // 在发送请求之前做些什么
    return config;
  },
  (error) => {
    // 对请求错误做些什么
    return Promise.reject(error);
  }
);

// 响应拦截器
axios.interceptors.response.use(
  (response) => {
    // 对响应数据做点什么
    return response;
  },
  (error) => {
    return Promise.reject(error);
  }
);

/**
 * GET 请求
 * @param { string } url 请求路径
 * @param { * } params 请求参数
 * @param { * } options 定制化请求参数
 */
export const get = (url, params, options) => axios.get(url, { params, ...options });

/**
 * POST 请求
 * @param { string } url 请求路径
 * @param { * } data 请求参数
 * @param { * } options 定制化请求参数
 */
export const post = (url, data, options) => axios.post(url, data, options);

/**
 * PUT 请求
 * @param { string } url 请求路径
 * @param { * } data 请求参数
 * @param { * } options 定制化请求参数
 */
export const put = (url, data, options) => axios.put(url, data, options);

/**
 * DELETE 请求
 * @param { string } url 请求路径
 * @param { * } data 请求参数
 * @param { * } options 定制化请求参数
 */
export const del = (url, params, options) => axios.delete(url, { params, ...options });
