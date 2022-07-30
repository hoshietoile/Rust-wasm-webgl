import { EV } from "~/@types/utils";

export const unwrapEV = <T extends HTMLInputElement>(ev: EV<T>) : string => {
  return ev?.target?.value || '';
}

export const parseIntOr = (str: string, defaultValue = 0, radix = 10) : number => {
  const tmp = parseInt(str, radix);
  return isNaN(tmp) ? defaultValue : tmp;
}