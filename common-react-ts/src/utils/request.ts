import axios, { AxiosRequestConfig, AxiosResponse, AxiosError } from 'axios';

axios.defaults.baseURL = 'http://localhost:3000';
axios.defaults.timeout = 5000;
axios.defaults.headers.common['Authorization'] = 'Bearer token';

// 请求拦截器
axios.interceptors.request.use(
  (config: AxiosRequestConfig) => {
    // 在发送请求之前做些什么
    return config;
  },
  (error: AxiosError) => {
    // 对请求错误做些什么
    return Promise.reject(error);
  }
);

// 响应拦截器
axios.interceptors.response.use(
  (response: AxiosResponse) => {
    // 对响应数据做点什么
    return response;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  }
);

/**
 * GET 请求
 * @param { string } url 请求路径
 * @param { Record<string, any> } params 请求参数
 * @param { AxiosRequestConfig } options 定制化请求参数
 * @returns { Promise<AxiosResponse> }
 */
export const get = (
  url: string,
  params?: Record<string, any>, // params 是可选的，类型为 object
  options?: AxiosRequestConfig
): Promise<AxiosResponse> => axios.get(url, { params, ...options });

/**
 * POST 请求
 * @param { string } url 请求路径
 * @param { any } data 请求参数
 * @param { AxiosRequestConfig } options 定制化请求参数
 * @returns { Promise<AxiosResponse> }
 */
export const post = (
  url: string,
  data: any,
  options?: AxiosRequestConfig
): Promise<AxiosResponse> => axios.post(url, data, options);

/**
 * PUT 请求
 * @param { string } url 请求路径
 * @param { any } data 请求参数
 * @param { AxiosRequestConfig } options 定制化请求参数
 * @returns { Promise<AxiosResponse> }
 */
export const put = (
  url: string,
  data: any,
  options?: AxiosRequestConfig
): Promise<AxiosResponse> => axios.put(url, data, options);

/**
 * DELETE 请求
 * @param { string } url 请求路径
 * @param { Record<string, any> } params 请求参数
 * @param { AxiosRequestConfig } options 定制化请求参数
 * @returns { Promise<AxiosResponse> }
 */
export const del = (
  url: string,
  params?: Record<string, any>,
  options?: AxiosRequestConfig
): Promise<AxiosResponse> => axios.delete(url, { params, ...options });
